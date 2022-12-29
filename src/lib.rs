use nom::{combinator::all_consuming, Finish};
use parse::{Name, NameMap, Valve};

mod parse;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Flow(u64);

type Connections = NameMap<(Path, Flow)>;

pub struct Network {
    valves: NameMap<(Valve, Connections)>,
}

pub type Path = Vec<(Name, Name)>;

impl Network {
    #![allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut net = Self {
            valves: Default::default(),
        };
        // let input = include_str!("input.txt");
        let input = include_str!("input-sample.txt");

        for valve in input
            .lines()
            .map(|l| all_consuming(parse::Valve::parse)(l).finish().unwrap().1)
        {
            net.valves.insert(valve.name, (valve, Default::default()));
        }

        let names = net.valves.keys().collect::<Vec<_>>();
        for name in names {
            let conns = net.connections(name);
            net.valves.get_mut(name).unwrap().1 = conns;
        }
        net
    }

    /// Given a valve name, return a list of valves we can travel to, along
    /// with the path to get there, and their flow. Excludes valves that
    /// have a flow of 0.
    ///
    /// Only the shortest paths are considered, so the search ends.
    fn connections(&self, start: Name) -> Connections {
        let mut current = Connections::default();
        {
            let valve = &self.valves.get(start).unwrap().0;
            current.insert(start, (vec![], Flow(valve.flow)));
        }

        let mut connections = current.clone();

        while !current.is_empty() {
            let mut next = Connections::default();
            for (name, (path, _flow)) in current.iter() {
                for link in self.valves.get(name).unwrap().0.links.iter().copied() {
                    let valve = &self.valves.get(link).unwrap().0;
                    if !connections.contains(link) {
                        let conn_path: Path = path
                            .iter()
                            .copied()
                            .chain(std::iter::once((name, link)))
                            .collect();
                        let item = (conn_path.clone(), Flow(valve.flow));
                        connections.insert(link, item.clone());
                        next.insert(link, item);
                    }
                }
            }
            current = next;
        }

        // filter out flow == 0
        let mut result = Connections::default();
        for (name, (path, flow)) in connections.into_iter() {
            if flow.0 > 0 {
                result.insert(name, (path, flow));
            }
        }
        result
    }
}

#[derive(Debug, Clone)]
struct Move<'a> {
    target: Name,
    path: &'a Path,
}

impl Move<'_> {
    fn cost(&self) -> u64 {
        let travel_turns = self.path.len() as u64;
        let open_turns = 1_u64;
        travel_turns + open_turns
    }
}

const NUM_ACTORS: usize = 1;

#[derive(Clone)]
pub struct State<'net> {
    net: &'net Network,
    max_turns: u64,
    turn: u64,
    pressure: u64,
    inner: StateInner,

    // 0 = ourselves, 1 = elephant
    actors: [Actor<'net>; NUM_ACTORS],
}

#[derive(Clone)]
struct StateInner {
    pressure_per_turn: u64,
    open_valves: NameMap<()>,
}

#[derive(Clone)]
struct Actor<'net> {
    position: Name,
    in_progress_move: Option<InProgressMove<'net>>,
}

impl<'net> Actor<'net> {
    fn step(&mut self, net: &'net Network, state: &mut StateInner) {
        if let Some(im) = self.in_progress_move.as_mut() {
            let target = im.mv.target;

            if im.turns + 1 >= im.mv.cost() {
                self.in_progress_move = None;
                self.position = target;

                state.open_valves.insert(target, ());
                state.pressure_per_turn += net.valves.get(target).unwrap().0.flow;
                self.in_progress_move = None;
            } else {
                let (_from, to) = im.mv.path[im.turns as usize];
                self.position = to;
                im.turns += 1;
            }
        }
    }
}

impl<'net> State<'net> {
    /// Compute all moves from given position
    fn moves(&self, position: Name) -> impl Iterator<Item = Move<'net>> + '_ {
        let (_valve, connections) = self.net.valves.get(position).unwrap();
        connections.iter().filter_map(|(name, (path, _flow))| {
            if self.inner.open_valves.contains(name) {
                return None;
            }
            Some(Move { target: name, path })
        })
    }
}

#[derive(Debug, Clone)]
struct InProgressMove<'net> {
    mv: Move<'net>,
    turns: u64,
}

impl<'net> InProgressMove<'net> {
    fn new(mv: Move<'net>) -> Self {
        Self { mv, turns: 0 }
    }
}

impl<'net> State<'net> {
    fn turns_left(&self) -> u64 {
        self.max_turns - self.turn
    }

    fn step(&mut self) -> bool {
        self.pressure += self.inner.pressure_per_turn;

        self.turn += 1;
        if self.turn >= self.max_turns {
            return false;
        }
        true
    }

    pub fn run_manual<'borrow>(&'borrow self) -> State<'net> {
        let mut state = self.clone();
        if !state.step() {
            // all done
            return state;
        }

        let actor_index = 0;

        let actor = &mut state.actors[actor_index];
        let mvs = if let Some(mv) = actor.in_progress_move.take() {
            itertools::Either::Left(std::iter::once(mv))
        } else {
            itertools::Either::Right(
                self.moves(actor.position)
                    .filter(|mv| mv.cost() <= state.turns_left())
                    .map(InProgressMove::new),
            )
        };

        // find best move
        {
            let mut best_state: Option<State<'net>> = None;

            for mv in mvs {
                let mut next: State<'net> = state.clone();
                let actor = &mut next.actors[0];
                actor.in_progress_move = Some(mv);
                actor.step(next.net, &mut next.inner);

                let end_state: State<'net> = next.run_manual();
                if end_state.pressure > best_state.as_ref().map(|s| s.pressure).unwrap_or_default()
                {
                    best_state = Some(end_state);
                }
            }

            match best_state {
                Some(state) => {
                    // we reached the end
                    state
                }
                None => {
                    // no moves possible, just wait it out
                    state.run_manual()
                }
            }
        }
    }

    pub fn run_max_by_key<'borrow>(&'borrow self) -> State<'net> {
        let mut state = self.clone();
        if !state.step() {
            // all done
            return state;
        }

        let actor_index = 0;

        let actor = &mut state.actors[actor_index];
        let mvs = if let Some(mv) = actor.in_progress_move.take() {
            itertools::Either::Left(std::iter::once(mv))
        } else {
            itertools::Either::Right(
                self.moves(actor.position)
                    .filter(|mv| mv.cost() <= state.turns_left())
                    .map(InProgressMove::new),
            )
        };

        mvs.map(|mv| {
            let mut next: State<'net> = state.clone();
            let actor = &mut next.actors[0];
            actor.in_progress_move = Some(mv);
            actor.step(next.net, &mut next.inner);
            next.run_max_by_key()
        })
        .max_by_key(|state| state.pressure)
        .unwrap_or_else(|| state.run_max_by_key())
    }
}

pub fn with_state(net: &Network, f: impl FnOnce(State)) {
    f(State {
        net,
        max_turns: 30,
        turn: 0,
        pressure: 0,
        inner: StateInner {
            pressure_per_turn: 0,
            open_valves: Default::default(),
        },
        actors: [Actor {
            in_progress_move: None,
            position: Name(*b"AA"),
        }],
    })
}
