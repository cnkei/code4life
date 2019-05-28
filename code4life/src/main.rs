use std::fmt;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

const MAX_SAMPLES: u8 = 3;
const MAX_MOLECULES: u8 = 10;

#[derive(Debug)]
enum Module {
    StartPosition, Samples, Diagnosis, Molecules, Laboratory,
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Module::StartPosition => write!(f, "START_POS"),
            Module::Samples => write!(f, "SAMPLES"),
            Module::Diagnosis => write!(f, "DIAGNOSIS"),
            Module::Molecules => write!(f, "MOLECULES"),
            Module::Laboratory => write!(f, "LABORATORY"),
        }
    }
}

#[derive(Debug)]
struct PlayerInfo {
    target: Module,
    eta: u32,
    score: u32,
    storage: [u8; 5],
    expertise: [u8; 5],
}

#[derive(Debug, PartialEq)]
enum Carrier {
    P1, P2, Cloud,
}

#[derive(Debug)]
struct SampleInfo {
    id: u32,
    carried_by: Carrier,
    rank: u8,
    diagnosed: bool,
    gain: String,
    health: i8,
    cost: [i8; 5],
}

#[derive(Debug)]
struct TurnInput {
    p1: PlayerInfo,
    p2: PlayerInfo,
    available: [u32; 5],
    samples: Vec<SampleInfo>,
}

fn get_player_info() -> PlayerInfo {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    PlayerInfo{
        target: match inputs[0].trim() {
            "START_POS" => Module::StartPosition,
            "SAMPLES" => Module::Samples,
            "DIAGNOSIS" => Module::Diagnosis,
            "MOLECULES" => Module::Molecules,
            "LABORATORY" => Module::Laboratory,
            _ => panic!(),
        },
        eta: parse_input!(inputs[1], u32),
        score: parse_input!(inputs[2], u32),
        storage: [
            parse_input!(inputs[3], u8),
            parse_input!(inputs[4], u8),
            parse_input!(inputs[5], u8),
            parse_input!(inputs[6], u8),
            parse_input!(inputs[7], u8),
        ],
        expertise: [
            parse_input!(inputs[8], u8),
            parse_input!(inputs[9], u8),
            parse_input!(inputs[10], u8),
            parse_input!(inputs[11], u8),
            parse_input!(inputs[12], u8),
        ],
    }
}

fn get_sample_info() -> SampleInfo {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let health = parse_input!(inputs[4], i8);
    SampleInfo{
        id: parse_input!(inputs[0], u32),
        carried_by: match inputs[1].trim() {
            "0" => Carrier::P1,
            "1" => Carrier::P2,
            "-1" => Carrier::Cloud,
            _ => panic!(),
        },
        rank: parse_input!(inputs[2], u8),
        gain: inputs[3].trim().to_string(),
        diagnosed: health != -1,
        health,
        cost: [
            parse_input!(inputs[5], i8),
            parse_input!(inputs[6], i8),
            parse_input!(inputs[7], i8),
            parse_input!(inputs[8], i8),
            parse_input!(inputs[9], i8),
        ],
    }
}

fn get_turn_input() -> TurnInput {
    let mut input_line = String::new();
    let p1 = get_player_info();
    let p2 = get_player_info();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let available = [
        parse_input!(inputs[0], u32),
        parse_input!(inputs[1], u32),
        parse_input!(inputs[2], u32),
        parse_input!(inputs[3], u32),
        parse_input!(inputs[4], u32),
    ];
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let sample_count = parse_input!(input_line, usize);
    let mut samples = Vec::with_capacity(sample_count);
    for _ in 0..sample_count as usize {
        samples.push(get_sample_info());
    }
    TurnInput{p1, p2, available, samples}
}

#[derive(Clone, Copy, Debug)]
enum Molecule {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
}

impl Molecule {
    fn from(id: u8) -> Self {
        match id {
            0 => Molecule::A,
            1 => Molecule::B,
            2 => Molecule::C,
            3 => Molecule::D,
            4 => Molecule::E,
            _ => panic!(),
        }
    }
}

impl fmt::Display for Molecule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (('A' as u8) + (*self as u8)) as char)
    }
}

#[derive(Debug)]
enum PlayerOp {
    GoTo(Module),
    ConnectRank(u8),
    ConnectSample(u32),
    ConnectMolecule(Molecule),
    Wait,
}

impl fmt::Display for PlayerOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlayerOp::GoTo(m) => write!(f, "GOTO {}", m),
            PlayerOp::ConnectRank(r) => write!(f, "CONNECT {}", r),
            PlayerOp::ConnectSample(s) => write!(f, "CONNECT {}", s),
            PlayerOp::ConnectMolecule(m) => write!(f, "CONNECT {}", m),
            PlayerOp::Wait => write!(f, "WAIT"),
        }
    }
}

fn work_samples(turn_input: &TurnInput) -> PlayerOp {
    let mut has_rank2 = false;
    let mut has_rank1 = false;
    for sample in turn_input.samples.iter() {
        match sample.carried_by {
            Carrier::P1 => {
                if sample.rank == 1 {
                    has_rank1 = true;
                } else if sample.rank == 2 {
                    has_rank2 = true;
                }
            },
            _ => continue,
        }
    }
    if !has_rank2 {
        return PlayerOp::ConnectRank(2);
    }
    if !has_rank1 {
        return PlayerOp::ConnectRank(1);
    }
    PlayerOp::GoTo(Module::Diagnosis)
}

fn work_diagnosis(turn_input: &TurnInput) -> PlayerOp {
    for sample in turn_input.samples.iter() {
        match sample.carried_by {
            Carrier::P1 => {
                if !sample.diagnosed {
                    return PlayerOp::ConnectSample(sample.id);
                }
            },
            _ => continue,
        }
    }
    PlayerOp::GoTo(Module::Molecules)
}

fn work_molecules(turn_input: &TurnInput) -> PlayerOp {
    let mut requires = vec![0; 5];
    for sample in turn_input.samples.iter() {
        if sample.carried_by == Carrier::P1 {
            for i in 0..5 {
                requires[i] += sample.cost[i];
            }
        }
    }
    if requires.iter().sum::<i8>() as u8 > MAX_MOLECULES {
        for sample in turn_input.samples.iter() {
            if sample.carried_by == Carrier::P1 && sample.rank == 2 {
                for i in 0..5 {
                    requires[i] = sample.cost[i];
                }
            }
        }
    }
    for i in 0..5 {
        if turn_input.p1.storage[i] < requires[i] as u8 {
            return PlayerOp::ConnectMolecule(Molecule::from(i as u8));
        }
    }
    PlayerOp::GoTo(Module::Laboratory)
}

fn work_laboratory(turn_input: &TurnInput) -> PlayerOp {
    for sample in turn_input.samples.iter() {
        if sample.carried_by == Carrier::P1 {
            if sample.rank == 2 || sample.cost.iter().sum::<i8>() as u8 == turn_input.p1.storage.iter().sum::<u8>() {
                return PlayerOp::ConnectSample(sample.id);
            }
        }
    }
    PlayerOp::GoTo(Module::Samples)
}

fn next_move(turn_input: &TurnInput) -> PlayerOp {
    match turn_input.p1.target {
        Module::StartPosition => PlayerOp::GoTo(Module::Samples),
        Module::Samples => work_samples(turn_input),
        Module::Diagnosis => work_diagnosis(turn_input),
        Module::Molecules => work_molecules(turn_input),
        Module::Laboratory => work_laboratory(turn_input),
    }
}

/**
 * Bring data on patient samples from the diagnosis machine to the laboratory with enough molecules to produce medicine!
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let project_count = parse_input!(input_line, i32);
    for _ in 0..project_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let a = parse_input!(inputs[0], i32);
        let b = parse_input!(inputs[1], i32);
        let c = parse_input!(inputs[2], i32);
        let d = parse_input!(inputs[3], i32);
        let e = parse_input!(inputs[4], i32);
    }

    // game loop
    loop {
        let turn_input = get_turn_input();
        println!("{}", next_move(&turn_input));
    }
}