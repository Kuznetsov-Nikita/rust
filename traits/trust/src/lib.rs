#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    BothCooperated,
    LeftCheated,
    RightCheated,
    BothCheated,
}

pub struct Game {
    left_player: Box<dyn Agent>,
    right_player: Box<dyn Agent>,
    left_player_score: i32,
    right_player_score: i32,
}

impl Game {
    pub fn new(left: Box<dyn Agent>, right: Box<dyn Agent>) -> Self {
        Self {
            left_player: left,
            right_player: right,
            left_player_score: 0,
            right_player_score: 0,
        }
    }

    pub fn left_score(&self) -> i32 {
        self.left_player_score
    }

    pub fn right_score(&self) -> i32 {
        self.right_player_score
    }

    pub fn play_round(&mut self) -> RoundOutcome {
        let left_player_decision = self.left_player.play();
        let right_player_decision = self.right_player.play();

        self.left_player
            .accept_enemy_decision(right_player_decision);
        self.right_player
            .accept_enemy_decision(left_player_decision);

        match (left_player_decision, right_player_decision) {
            (Decision::Cooperated, Decision::Cooperated) => {
                self.left_player_score += 2;
                self.right_player_score += 2;
                RoundOutcome::BothCooperated
            }
            (Decision::Cheeted, Decision::Cooperated) => {
                self.left_player_score += 3;
                self.right_player_score -= 1;
                RoundOutcome::LeftCheated
            }
            (Decision::Cooperated, Decision::Cheeted) => {
                self.left_player_score -= 1;
                self.right_player_score += 3;
                RoundOutcome::RightCheated
            }
            (Decision::Cheeted, Decision::Cheeted) => RoundOutcome::BothCheated,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Decision {
    #[default]
    Cooperated,
    Cheeted,
}

pub trait Agent {
    fn play(&mut self) -> Decision;

    fn accept_enemy_decision(&mut self, _decision: Decision) {}
}

#[derive(Default)]
pub struct CheatingAgent {}

impl Agent for CheatingAgent {
    fn play(&mut self) -> Decision {
        Decision::Cheeted
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CooperatingAgent {}

impl Agent for CooperatingAgent {
    fn play(&mut self) -> Decision {
        Decision::Cooperated
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct GrudgerAgent {
    was_decived: bool,
}

impl GrudgerAgent {
    pub fn new() -> Self {
        Self { was_decived: false }
    }

    pub fn decive(&mut self) {
        self.was_decived = true;
    }
}

impl Agent for GrudgerAgent {
    fn play(&mut self) -> Decision {
        if !self.was_decived {
            Decision::Cooperated
        } else {
            Decision::Cheeted
        }
    }

    fn accept_enemy_decision(&mut self, decision: Decision) {
        if decision == Decision::Cheeted {
            self.decive()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CopycatAgent {
    previous_enemy_decision: Decision,
}

impl CopycatAgent {
    pub fn new() -> Self {
        Self {
            previous_enemy_decision: Decision::Cooperated,
        }
    }

    pub fn set_enemy_decision(&mut self, decision: Decision) {
        self.previous_enemy_decision = decision
    }
}

impl Agent for CopycatAgent {
    fn play(&mut self) -> Decision {
        self.previous_enemy_decision
    }

    fn accept_enemy_decision(&mut self, decision: Decision) {
        self.set_enemy_decision(decision)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct DetectiveAgent {
    move_number: u32,
    was_decived: bool,
    my_next_decision: Decision,
}

impl DetectiveAgent {
    pub fn new() -> Self {
        Self {
            move_number: 0,
            was_decived: false,
            my_next_decision: Decision::Cooperated,
        }
    }

    pub fn decive(&mut self) {
        self.was_decived = true
    }

    pub fn set_enemy_decision(&mut self, decision: Decision) {
        if self.was_decived {
            self.my_next_decision = decision
        }
    }
}

impl Agent for DetectiveAgent {
    fn play(&mut self) -> Decision {
        self.move_number += 1;

        match self.move_number {
            1 => Decision::Cooperated,
            2 => Decision::Cheeted,
            3 => Decision::Cooperated,
            4 => Decision::Cooperated,
            _ => {
                if !self.was_decived {
                    Decision::Cheeted
                } else {
                    self.my_next_decision
                }
            }
        }
    }

    fn accept_enemy_decision(&mut self, decision: Decision) {
        match self.move_number {
            1 | 2 | 3 | 4 => {
                if decision == Decision::Cheeted {
                    self.decive()
                }
                self.set_enemy_decision(decision)
            }
            _ => self.set_enemy_decision(decision),
        }
    }
}
