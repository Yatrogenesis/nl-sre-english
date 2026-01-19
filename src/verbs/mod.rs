//! # English Verb Database
//!
//! Comprehensive database of English verbs organized by functional categories.
//! Designed for easy programmatic access and semantic understanding.
//!
//! ## Functional Categories (25+)
//!
//! - **Movement**: walk, run, go, come, move, travel...
//! - **Perception**: see, hear, feel, smell, taste, watch...
//! - **Communication**: say, tell, speak, ask, answer, explain...
//! - **Cognition**: think, know, believe, understand, remember...
//! - **Emotion**: love, hate, fear, hope, enjoy, suffer...
//! - **Physical**: hit, cut, push, pull, grab, throw...
//! - **State**: be, have, exist, remain, stay, seem...
//! - **Change**: become, grow, turn, transform, evolve...
//! - **Transfer**: give, take, send, receive, deliver...
//! - **Creation**: make, create, build, produce, generate...
//! - **Destruction**: destroy, break, kill, eliminate, ruin...
//! - **Control**: control, manage, lead, direct, govern...
//! - **Possession**: own, possess, belong, acquire, lose...
//! - **Social**: meet, help, cooperate, compete, fight...
//! - **Consumption**: eat, drink, consume, absorb, digest...
//! - And 10+ more categories...

use std::collections::HashMap;

mod data;
mod data2;
mod data3;
mod data4;

/// Functional category for verbs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FunctionalCategory {
    /// Movement and locomotion (walk, run, fly, swim)
    Movement,
    /// Perception and sensing (see, hear, feel, smell)
    Perception,
    /// Communication and speech (say, tell, speak, ask)
    Communication,
    /// Mental processes (think, know, believe, understand)
    Cognition,
    /// Emotional states and reactions (love, hate, fear)
    Emotion,
    /// Physical actions on objects (hit, cut, push, pull)
    Physical,
    /// State of being (be, exist, remain, stay)
    State,
    /// Change of state (become, grow, transform)
    Change,
    /// Transfer of possession (give, take, send, receive)
    Transfer,
    /// Creation and production (make, create, build)
    Creation,
    /// Destruction and damage (destroy, break, kill)
    Destruction,
    /// Control and management (control, manage, lead)
    Control,
    /// Possession and ownership (own, have, possess)
    Possession,
    /// Social interaction (meet, help, fight, cooperate)
    Social,
    /// Eating and drinking (eat, drink, consume)
    Consumption,
    /// Body functions (breathe, sleep, wake, rest)
    Body,
    /// Weather and natural phenomena (rain, snow, shine)
    Weather,
    /// Measurement and comparison (measure, weigh, compare)
    Measurement,
    /// Start and stop (begin, start, stop, end, finish)
    Aspectual,
    /// Causation (cause, make, let, allow, force)
    Causation,
    /// Attempt and success (try, attempt, succeed, fail)
    Attempt,
    /// Modal-like verbs (can, must, should, need, want)
    Modal,
    /// Position and location (sit, stand, lie, hang)
    Position,
    /// Connection and separation (connect, join, separate, divide)
    Connection,
    /// Light and sound emission (shine, glow, ring, buzz)
    Emission,
}

impl FunctionalCategory {
    /// Get all categories
    pub fn all() -> &'static [FunctionalCategory] {
        &[
            FunctionalCategory::Movement,
            FunctionalCategory::Perception,
            FunctionalCategory::Communication,
            FunctionalCategory::Cognition,
            FunctionalCategory::Emotion,
            FunctionalCategory::Physical,
            FunctionalCategory::State,
            FunctionalCategory::Change,
            FunctionalCategory::Transfer,
            FunctionalCategory::Creation,
            FunctionalCategory::Destruction,
            FunctionalCategory::Control,
            FunctionalCategory::Possession,
            FunctionalCategory::Social,
            FunctionalCategory::Consumption,
            FunctionalCategory::Body,
            FunctionalCategory::Weather,
            FunctionalCategory::Measurement,
            FunctionalCategory::Aspectual,
            FunctionalCategory::Causation,
            FunctionalCategory::Attempt,
            FunctionalCategory::Modal,
            FunctionalCategory::Position,
            FunctionalCategory::Connection,
            FunctionalCategory::Emission,
        ]
    }

    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            FunctionalCategory::Movement => "Movement",
            FunctionalCategory::Perception => "Perception",
            FunctionalCategory::Communication => "Communication",
            FunctionalCategory::Cognition => "Cognition",
            FunctionalCategory::Emotion => "Emotion",
            FunctionalCategory::Physical => "Physical Action",
            FunctionalCategory::State => "State",
            FunctionalCategory::Change => "Change of State",
            FunctionalCategory::Transfer => "Transfer",
            FunctionalCategory::Creation => "Creation",
            FunctionalCategory::Destruction => "Destruction",
            FunctionalCategory::Control => "Control",
            FunctionalCategory::Possession => "Possession",
            FunctionalCategory::Social => "Social",
            FunctionalCategory::Consumption => "Consumption",
            FunctionalCategory::Body => "Body Function",
            FunctionalCategory::Weather => "Weather",
            FunctionalCategory::Measurement => "Measurement",
            FunctionalCategory::Aspectual => "Aspectual",
            FunctionalCategory::Causation => "Causation",
            FunctionalCategory::Attempt => "Attempt",
            FunctionalCategory::Modal => "Modal",
            FunctionalCategory::Position => "Position",
            FunctionalCategory::Connection => "Connection",
            FunctionalCategory::Emission => "Emission",
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            FunctionalCategory::Movement => "Verbs describing motion and locomotion",
            FunctionalCategory::Perception => "Verbs of sensing and perceiving",
            FunctionalCategory::Communication => "Verbs of speaking and communication",
            FunctionalCategory::Cognition => "Verbs of thinking and mental processes",
            FunctionalCategory::Emotion => "Verbs expressing emotional states",
            FunctionalCategory::Physical => "Verbs of physical manipulation",
            FunctionalCategory::State => "Verbs describing states of being",
            FunctionalCategory::Change => "Verbs describing change of state",
            FunctionalCategory::Transfer => "Verbs of giving and receiving",
            FunctionalCategory::Creation => "Verbs of making and producing",
            FunctionalCategory::Destruction => "Verbs of breaking and destroying",
            FunctionalCategory::Control => "Verbs of controlling and managing",
            FunctionalCategory::Possession => "Verbs of owning and having",
            FunctionalCategory::Social => "Verbs of social interaction",
            FunctionalCategory::Consumption => "Verbs of eating and drinking",
            FunctionalCategory::Body => "Verbs of bodily functions",
            FunctionalCategory::Weather => "Verbs describing weather phenomena",
            FunctionalCategory::Measurement => "Verbs of measuring and comparing",
            FunctionalCategory::Aspectual => "Verbs marking beginning, ending, continuation",
            FunctionalCategory::Causation => "Verbs of causing and enabling",
            FunctionalCategory::Attempt => "Verbs of trying and succeeding/failing",
            FunctionalCategory::Modal => "Modal and semi-modal verbs",
            FunctionalCategory::Position => "Verbs of body position and location",
            FunctionalCategory::Connection => "Verbs of joining and separating",
            FunctionalCategory::Emission => "Verbs of light and sound emission",
        }
    }
}

/// More specific verb group within a category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerbGroup {
    // Movement subgroups
    Walk,           // walk, stroll, march, pace
    Run,            // run, sprint, dash, race
    Jump,           // jump, leap, hop, skip
    Fly,            // fly, soar, glide, hover
    Swim,           // swim, dive, float, wade
    Climb,          // climb, ascend, scale, mount
    Fall,           // fall, drop, tumble, collapse
    Turn,           // turn, rotate, spin, twist
    Enter,          // enter, arrive, come, approach
    Exit,           // exit, leave, depart, go

    // Perception subgroups
    See,            // see, watch, observe, notice
    Hear,           // hear, listen, overhear
    Feel,           // feel, touch, sense
    Smell,          // smell, sniff, scent
    Taste,          // taste, sample, savor

    // Communication subgroups
    Speak,          // speak, talk, say, tell
    Ask,            // ask, question, inquire, request
    Answer,         // answer, reply, respond
    Explain,        // explain, describe, clarify
    Argue,          // argue, debate, discuss, dispute
    Promise,        // promise, vow, swear, pledge
    Warn,           // warn, caution, alert, advise
    Command,        // command, order, instruct, direct
    Suggest,        // suggest, propose, recommend

    // Cognition subgroups
    Think,          // think, ponder, consider, contemplate
    Know,           // know, understand, comprehend, grasp
    Believe,        // believe, suppose, assume, presume
    Remember,       // remember, recall, recollect
    Forget,         // forget, overlook, neglect
    Learn,          // learn, study, discover, realize
    Decide,         // decide, choose, determine, resolve
    Plan,           // plan, intend, aim, scheme
    Imagine,        // imagine, envision, visualize, dream
    Analyze,        // analyze, examine, investigate, evaluate

    // Emotion subgroups
    Love,           // love, adore, cherish, treasure
    Hate,           // hate, detest, loathe, despise
    Fear,           // fear, dread, worry, frighten
    Hope,           // hope, wish, desire, long
    Enjoy,          // enjoy, like, appreciate, relish
    Suffer,         // suffer, hurt, ache, grieve
    Surprise,       // surprise, shock, amaze, astonish
    Anger,          // anger, annoy, irritate, enrage
    Satisfy,        // satisfy, please, content, gratify

    // Physical action subgroups
    Hit,            // hit, strike, punch, beat
    Cut,            // cut, slice, chop, carve
    Push,           // push, shove, press, thrust
    Pull,           // pull, drag, tug, haul
    Throw,          // throw, toss, hurl, cast
    Catch,          // catch, grab, seize, capture
    Hold,           // hold, grasp, grip, clutch
    Lift,           // lift, raise, elevate, hoist
    Open,           // open, unlock, unfold, uncover
    Close,          // close, shut, seal, lock
    Touch,          // touch, tap, poke, stroke
    Kick,           // kick, stomp, trample, stamp

    // State subgroups
    Be,             // be, exist
    Have,           // have, possess
    Seem,           // seem, appear, look
    Remain,         // remain, stay, persist, continue
    Become,         // become, turn, grow, get

    // Transfer subgroups
    Give,           // give, donate, grant, offer
    Take,           // take, grab, seize, acquire
    Send,           // send, deliver, ship, transmit
    Receive,        // receive, get, obtain, accept
    Lend,           // lend, loan, advance
    Borrow,         // borrow, rent, lease
    Steal,          // steal, rob, take, pilfer
    Return,         // return, restore, give back

    // Creation subgroups
    Make,           // make, produce, manufacture
    Create,         // create, invent, design, develop
    Build,          // build, construct, erect, assemble
    Write,          // write, compose, author, draft
    Draw,           // draw, sketch, paint, illustrate
    Cook,           // cook, bake, fry, prepare
    Grow,           // grow, cultivate, raise, plant

    // Destruction subgroups
    Destroy,        // destroy, demolish, wreck, annihilate
    Break,          // break, shatter, crack, smash
    Kill,           // kill, murder, slay, eliminate
    Damage,         // damage, harm, injure, hurt
    Burn,           // burn, incinerate, scorch
    Erase,          // erase, delete, remove, wipe

    // Control subgroups
    ControlGroup,   // control, manage, handle, operate
    Lead,           // lead, guide, direct, conduct
    Govern,         // govern, rule, reign, administer
    Supervise,      // supervise, oversee, monitor, watch
    Influence,      // influence, affect, impact, sway

    // Possession subgroups
    Own,            // own, possess, hold, have
    Acquire,        // acquire, obtain, gain, get
    Lose,           // lose, misplace, forfeit
    Keep,           // keep, retain, maintain, preserve
    Share,          // share, distribute, divide

    // Social subgroups
    Meet,           // meet, encounter, greet
    Help,           // help, assist, aid, support
    Fight,          // fight, battle, combat, struggle
    Cooperate,      // cooperate, collaborate, work together
    Compete,        // compete, rival, contest
    Follow,         // follow, pursue, chase, track
    Obey,           // obey, comply, submit, yield
    Resist,         // resist, oppose, defy, rebel

    // Consumption subgroups
    Eat,            // eat, devour, consume, swallow
    Drink,          // drink, sip, gulp, swallow
    Breathe,        // breathe, inhale, exhale, respire

    // Body subgroups
    Sleep,          // sleep, nap, doze, rest
    Wake,           // wake, awaken, arise, stir
    Sit,            // sit, seat, perch
    Stand,          // stand, rise, get up
    Lie,            // lie, recline, lay
    Kneel,          // kneel, crouch, squat
    Bend,           // bend, bow, stoop, lean

    // Aspectual subgroups
    Begin,          // begin, start, commence, initiate
    End,            // end, finish, complete, conclude
    Continue,       // continue, proceed, go on, persist
    Stop,           // stop, halt, cease, pause
    Repeat,         // repeat, redo, reiterate

    // Causation subgroups
    Cause,          // cause, make, bring about
    Allow,          // allow, permit, let, enable
    Prevent,        // prevent, stop, block, hinder
    Force,          // force, compel, oblige, coerce
    HelpCausation, // help, assist, aid, facilitate

    // Attempt subgroups
    Try,            // try, attempt, endeavor
    Succeed,        // succeed, accomplish, achieve
    Fail,           // fail, flunk, miss, lose
    Practice,       // practice, rehearse, train

    // Connection subgroups
    Connect,        // connect, join, link, attach
    Separate,       // separate, divide, split, part
    Combine,        // combine, merge, mix, blend
    Attach,         // attach, fasten, secure, fix

    // Emission subgroups
    Shine,          // shine, glow, gleam, radiate
    Sound,          // sound, ring, buzz, chime

    // Measurement
    Measure,        // measure, weigh, calculate
    Compare,        // compare, contrast, match
    Count,          // count, number, tally

    // Position
    Put,            // put, place, set, lay
    Remove,         // remove, take away, extract

    // Weather (impersonal)
    Rain,           // rain, pour, drizzle
    Snow,           // snow, blizzard
    Blow,           // blow (wind)

    // Modal
    Want,           // want, wish, desire
    Need,           // need, require, must
    Can,            // can, be able to
    Should,         // should, ought to

    // Generic fallback
    Generic,
}

impl VerbGroup {
    /// Get the functional category this group belongs to
    pub fn category(&self) -> FunctionalCategory {
        match self {
            // Movement
            VerbGroup::Walk | VerbGroup::Run | VerbGroup::Jump | VerbGroup::Fly |
            VerbGroup::Swim | VerbGroup::Climb | VerbGroup::Fall | VerbGroup::Turn |
            VerbGroup::Enter | VerbGroup::Exit => FunctionalCategory::Movement,

            // Perception
            VerbGroup::See | VerbGroup::Hear | VerbGroup::Feel |
            VerbGroup::Smell | VerbGroup::Taste => FunctionalCategory::Perception,

            // Communication
            VerbGroup::Speak | VerbGroup::Ask | VerbGroup::Answer | VerbGroup::Explain |
            VerbGroup::Argue | VerbGroup::Promise | VerbGroup::Warn |
            VerbGroup::Command | VerbGroup::Suggest => FunctionalCategory::Communication,

            // Cognition
            VerbGroup::Think | VerbGroup::Know | VerbGroup::Believe | VerbGroup::Remember |
            VerbGroup::Forget | VerbGroup::Learn | VerbGroup::Decide | VerbGroup::Plan |
            VerbGroup::Imagine | VerbGroup::Analyze => FunctionalCategory::Cognition,

            // Emotion
            VerbGroup::Love | VerbGroup::Hate | VerbGroup::Fear | VerbGroup::Hope |
            VerbGroup::Enjoy | VerbGroup::Suffer | VerbGroup::Surprise |
            VerbGroup::Anger | VerbGroup::Satisfy => FunctionalCategory::Emotion,

            // Physical
            VerbGroup::Hit | VerbGroup::Cut | VerbGroup::Push | VerbGroup::Pull |
            VerbGroup::Throw | VerbGroup::Catch | VerbGroup::Hold | VerbGroup::Lift |
            VerbGroup::Open | VerbGroup::Close | VerbGroup::Touch |
            VerbGroup::Kick => FunctionalCategory::Physical,

            // State
            VerbGroup::Be | VerbGroup::Have | VerbGroup::Seem |
            VerbGroup::Remain | VerbGroup::Become => FunctionalCategory::State,

            // Transfer
            VerbGroup::Give | VerbGroup::Take | VerbGroup::Send | VerbGroup::Receive |
            VerbGroup::Lend | VerbGroup::Borrow | VerbGroup::Steal |
            VerbGroup::Return => FunctionalCategory::Transfer,

            // Creation
            VerbGroup::Make | VerbGroup::Create | VerbGroup::Build | VerbGroup::Write |
            VerbGroup::Draw | VerbGroup::Cook | VerbGroup::Grow => FunctionalCategory::Creation,

            // Destruction
            VerbGroup::Destroy | VerbGroup::Break | VerbGroup::Kill |
            VerbGroup::Damage | VerbGroup::Burn | VerbGroup::Erase => FunctionalCategory::Destruction,

            // Control
            VerbGroup::ControlGroup | VerbGroup::Lead | VerbGroup::Govern |
            VerbGroup::Supervise | VerbGroup::Influence => FunctionalCategory::Control,

            // Possession
            VerbGroup::Own | VerbGroup::Acquire | VerbGroup::Lose |
            VerbGroup::Keep | VerbGroup::Share => FunctionalCategory::Possession,

            // Social
            VerbGroup::Meet | VerbGroup::Help | VerbGroup::Fight | VerbGroup::Cooperate |
            VerbGroup::Compete | VerbGroup::Follow | VerbGroup::Obey |
            VerbGroup::Resist => FunctionalCategory::Social,

            // Consumption
            VerbGroup::Eat | VerbGroup::Drink | VerbGroup::Breathe => FunctionalCategory::Consumption,

            // Body
            VerbGroup::Sleep | VerbGroup::Wake | VerbGroup::Sit | VerbGroup::Stand |
            VerbGroup::Lie | VerbGroup::Kneel | VerbGroup::Bend => FunctionalCategory::Body,

            // Aspectual
            VerbGroup::Begin | VerbGroup::End | VerbGroup::Continue |
            VerbGroup::Stop | VerbGroup::Repeat => FunctionalCategory::Aspectual,

            // Causation
            VerbGroup::Cause | VerbGroup::Allow | VerbGroup::Prevent |
            VerbGroup::Force | VerbGroup::HelpCausation => FunctionalCategory::Causation,

            // Attempt
            VerbGroup::Try | VerbGroup::Succeed | VerbGroup::Fail |
            VerbGroup::Practice => FunctionalCategory::Attempt,

            // Connection
            VerbGroup::Connect | VerbGroup::Separate | VerbGroup::Combine |
            VerbGroup::Attach => FunctionalCategory::Connection,

            // Emission
            VerbGroup::Shine | VerbGroup::Sound => FunctionalCategory::Emission,

            // Measurement
            VerbGroup::Measure | VerbGroup::Compare | VerbGroup::Count => FunctionalCategory::Measurement,

            // Position
            VerbGroup::Put | VerbGroup::Remove => FunctionalCategory::Position,

            // Weather
            VerbGroup::Rain | VerbGroup::Snow | VerbGroup::Blow => FunctionalCategory::Weather,

            // Modal
            VerbGroup::Want | VerbGroup::Need | VerbGroup::Can |
            VerbGroup::Should => FunctionalCategory::Modal,

            VerbGroup::Generic => FunctionalCategory::State,
        }
    }

    /// Get the name of this group
    pub fn name(&self) -> &'static str {
        match self {
            VerbGroup::Walk => "Walk",
            VerbGroup::Run => "Run",
            VerbGroup::Jump => "Jump",
            VerbGroup::Fly => "Fly",
            VerbGroup::Swim => "Swim",
            VerbGroup::Climb => "Climb",
            VerbGroup::Fall => "Fall",
            VerbGroup::Turn => "Turn",
            VerbGroup::Enter => "Enter",
            VerbGroup::Exit => "Exit",
            VerbGroup::See => "See",
            VerbGroup::Hear => "Hear",
            VerbGroup::Feel => "Feel",
            VerbGroup::Smell => "Smell",
            VerbGroup::Taste => "Taste",
            VerbGroup::Speak => "Speak",
            VerbGroup::Ask => "Ask",
            VerbGroup::Answer => "Answer",
            VerbGroup::Explain => "Explain",
            VerbGroup::Argue => "Argue",
            VerbGroup::Promise => "Promise",
            VerbGroup::Warn => "Warn",
            VerbGroup::Command => "Command",
            VerbGroup::Suggest => "Suggest",
            VerbGroup::Think => "Think",
            VerbGroup::Know => "Know",
            VerbGroup::Believe => "Believe",
            VerbGroup::Remember => "Remember",
            VerbGroup::Forget => "Forget",
            VerbGroup::Learn => "Learn",
            VerbGroup::Decide => "Decide",
            VerbGroup::Plan => "Plan",
            VerbGroup::Imagine => "Imagine",
            VerbGroup::Analyze => "Analyze",
            VerbGroup::Love => "Love",
            VerbGroup::Hate => "Hate",
            VerbGroup::Fear => "Fear",
            VerbGroup::Hope => "Hope",
            VerbGroup::Enjoy => "Enjoy",
            VerbGroup::Suffer => "Suffer",
            VerbGroup::Surprise => "Surprise",
            VerbGroup::Anger => "Anger",
            VerbGroup::Satisfy => "Satisfy",
            VerbGroup::Hit => "Hit",
            VerbGroup::Cut => "Cut",
            VerbGroup::Push => "Push",
            VerbGroup::Pull => "Pull",
            VerbGroup::Throw => "Throw",
            VerbGroup::Catch => "Catch",
            VerbGroup::Hold => "Hold",
            VerbGroup::Lift => "Lift",
            VerbGroup::Open => "Open",
            VerbGroup::Close => "Close",
            VerbGroup::Touch => "Touch",
            VerbGroup::Kick => "Kick",
            VerbGroup::Be => "Be",
            VerbGroup::Have => "Have",
            VerbGroup::Seem => "Seem",
            VerbGroup::Remain => "Remain",
            VerbGroup::Become => "Become",
            VerbGroup::Give => "Give",
            VerbGroup::Take => "Take",
            VerbGroup::Send => "Send",
            VerbGroup::Receive => "Receive",
            VerbGroup::Lend => "Lend",
            VerbGroup::Borrow => "Borrow",
            VerbGroup::Steal => "Steal",
            VerbGroup::Return => "Return",
            VerbGroup::Make => "Make",
            VerbGroup::Create => "Create",
            VerbGroup::Build => "Build",
            VerbGroup::Write => "Write",
            VerbGroup::Draw => "Draw",
            VerbGroup::Cook => "Cook",
            VerbGroup::Grow => "Grow",
            VerbGroup::Destroy => "Destroy",
            VerbGroup::Break => "Break",
            VerbGroup::Kill => "Kill",
            VerbGroup::Damage => "Damage",
            VerbGroup::Burn => "Burn",
            VerbGroup::Erase => "Erase",
            VerbGroup::ControlGroup => "Control",
            VerbGroup::Lead => "Lead",
            VerbGroup::Govern => "Govern",
            VerbGroup::Supervise => "Supervise",
            VerbGroup::Influence => "Influence",
            VerbGroup::Own => "Own",
            VerbGroup::Acquire => "Acquire",
            VerbGroup::Lose => "Lose",
            VerbGroup::Keep => "Keep",
            VerbGroup::Share => "Share",
            VerbGroup::Meet => "Meet",
            VerbGroup::Help => "Help",
            VerbGroup::Fight => "Fight",
            VerbGroup::Cooperate => "Cooperate",
            VerbGroup::Compete => "Compete",
            VerbGroup::Follow => "Follow",
            VerbGroup::Obey => "Obey",
            VerbGroup::Resist => "Resist",
            VerbGroup::Eat => "Eat",
            VerbGroup::Drink => "Drink",
            VerbGroup::Breathe => "Breathe",
            VerbGroup::Sleep => "Sleep",
            VerbGroup::Wake => "Wake",
            VerbGroup::Sit => "Sit",
            VerbGroup::Stand => "Stand",
            VerbGroup::Lie => "Lie",
            VerbGroup::Kneel => "Kneel",
            VerbGroup::Bend => "Bend",
            VerbGroup::Begin => "Begin",
            VerbGroup::End => "End",
            VerbGroup::Continue => "Continue",
            VerbGroup::Stop => "Stop",
            VerbGroup::Repeat => "Repeat",
            VerbGroup::Cause => "Cause",
            VerbGroup::Allow => "Allow",
            VerbGroup::Prevent => "Prevent",
            VerbGroup::Force => "Force",
            VerbGroup::HelpCausation => "Help (Causative)",
            VerbGroup::Try => "Try",
            VerbGroup::Succeed => "Succeed",
            VerbGroup::Fail => "Fail",
            VerbGroup::Practice => "Practice",
            VerbGroup::Connect => "Connect",
            VerbGroup::Separate => "Separate",
            VerbGroup::Combine => "Combine",
            VerbGroup::Attach => "Attach",
            VerbGroup::Shine => "Shine",
            VerbGroup::Sound => "Sound",
            VerbGroup::Measure => "Measure",
            VerbGroup::Compare => "Compare",
            VerbGroup::Count => "Count",
            VerbGroup::Put => "Put",
            VerbGroup::Remove => "Remove",
            VerbGroup::Rain => "Rain",
            VerbGroup::Snow => "Snow",
            VerbGroup::Blow => "Blow",
            VerbGroup::Want => "Want",
            VerbGroup::Need => "Need",
            VerbGroup::Can => "Can",
            VerbGroup::Should => "Should",
            VerbGroup::Generic => "Generic",
        }
    }
}

/// A single verb entry
#[derive(Debug, Clone)]
pub struct VerbEntry {
    /// Base form (infinitive)
    pub base: String,
    /// Past tense
    pub past: String,
    /// Past participle
    pub past_participle: String,
    /// Present participle (-ing)
    pub present_participle: String,
    /// Third person singular (he/she/it form)
    pub third_person: String,
    /// Functional category
    pub category: FunctionalCategory,
    /// Specific verb group
    pub group: VerbGroup,
    /// Is irregular?
    pub irregular: bool,
    /// Transitivity: true = transitive, false = intransitive, None = both
    pub transitive: Option<bool>,
    /// Frequency (higher = more common, 1-100)
    pub frequency: u8,
    /// Related verbs (synonyms)
    pub synonyms: Vec<String>,
    /// Opposite verbs (antonyms)
    pub antonyms: Vec<String>,
}

impl VerbEntry {
    /// Create a new regular verb entry
    pub fn regular(
        base: &str,
        category: FunctionalCategory,
        group: VerbGroup,
        transitive: Option<bool>,
        frequency: u8,
    ) -> Self {
        let base_s = base.to_string();
        Self {
            past: Self::regular_past(&base_s),
            past_participle: Self::regular_past(&base_s),
            present_participle: Self::regular_ing(&base_s),
            third_person: Self::regular_third(&base_s),
            base: base_s,
            category,
            group,
            irregular: false,
            transitive,
            frequency,
            synonyms: Vec::new(),
            antonyms: Vec::new(),
        }
    }

    /// Create an irregular verb entry
    pub fn irregular(
        base: &str,
        past: &str,
        past_participle: &str,
        category: FunctionalCategory,
        group: VerbGroup,
        transitive: Option<bool>,
        frequency: u8,
    ) -> Self {
        let base_s = base.to_string();
        Self {
            past: past.to_string(),
            past_participle: past_participle.to_string(),
            present_participle: Self::regular_ing(&base_s),
            third_person: Self::regular_third(&base_s),
            base: base_s,
            category,
            group,
            irregular: true,
            transitive,
            frequency,
            synonyms: Vec::new(),
            antonyms: Vec::new(),
        }
    }

    /// Add synonyms
    pub fn with_synonyms(mut self, synonyms: &[&str]) -> Self {
        self.synonyms = synonyms.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Add antonyms
    pub fn with_antonyms(mut self, antonyms: &[&str]) -> Self {
        self.antonyms = antonyms.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Get regular past tense
    fn regular_past(base: &str) -> String {
        if base.ends_with('e') {
            format!("{}d", base)
        } else if base.ends_with('y') && base.len() > 1 {
            let chars: Vec<char> = base.chars().collect();
            let second_last = chars.get(chars.len() - 2);
            if second_last.map(|c| !"aeiou".contains(*c)).unwrap_or(false) {
                format!("{}ied", &base[..base.len()-1])
            } else {
                format!("{}ed", base)
            }
        } else if Self::should_double_final(base) {
            format!("{}{}ed", base, base.chars().last().unwrap())
        } else {
            format!("{}ed", base)
        }
    }

    /// Get regular -ing form
    fn regular_ing(base: &str) -> String {
        if base.ends_with('e') && !base.ends_with("ee") {
            format!("{}ing", &base[..base.len()-1])
        } else if base.ends_with("ie") {
            format!("{}ying", &base[..base.len()-2])
        } else if Self::should_double_final(base) {
            format!("{}{}ing", base, base.chars().last().unwrap())
        } else {
            format!("{}ing", base)
        }
    }

    /// Get regular third person singular
    fn regular_third(base: &str) -> String {
        if base.ends_with('s') || base.ends_with('x') || base.ends_with('z')
            || base.ends_with("ch") || base.ends_with("sh") {
            format!("{}es", base)
        } else if base.ends_with('y') && base.len() > 1 {
            let chars: Vec<char> = base.chars().collect();
            let second_last = chars.get(chars.len() - 2);
            if second_last.map(|c| !"aeiou".contains(*c)).unwrap_or(false) {
                format!("{}ies", &base[..base.len()-1])
            } else {
                format!("{}s", base)
            }
        } else {
            format!("{}s", base)
        }
    }

    /// Check if final consonant should be doubled
    fn should_double_final(base: &str) -> bool {
        let chars: Vec<char> = base.chars().collect();
        if chars.len() < 2 {
            return false;
        }
        let last = chars[chars.len() - 1];
        let second_last = chars[chars.len() - 2];

        // Final consonant after single vowel in stressed syllable
        !"aeiou".contains(last) && "aeiou".contains(second_last)
            && chars.len() <= 3 // Simple heuristic for short words
            && !['w', 'x', 'y'].contains(&last)
    }

    /// Check if a word form matches this verb
    pub fn matches(&self, word: &str) -> bool {
        let w = word.to_lowercase();
        w == self.base || w == self.past || w == self.past_participle
            || w == self.present_participle || w == self.third_person
    }
}

/// The complete verb database
#[derive(Debug)]
pub struct VerbDatabase {
    /// All verb entries by base form
    verbs: HashMap<String, VerbEntry>,
    /// Index: all forms -> base form
    form_index: HashMap<String, String>,
    /// Index: category -> verb bases
    category_index: HashMap<FunctionalCategory, Vec<String>>,
    /// Index: group -> verb bases
    group_index: HashMap<VerbGroup, Vec<String>>,
    /// Statistics
    pub stats: VerbStats,
}

/// Database statistics
#[derive(Debug, Clone, Default)]
pub struct VerbStats {
    pub total_verbs: usize,
    pub irregular_verbs: usize,
    pub regular_verbs: usize,
    pub total_forms: usize,
    pub categories_used: usize,
    pub groups_used: usize,
}

impl VerbDatabase {
    /// Create a new empty database
    pub fn new() -> Self {
        Self {
            verbs: HashMap::new(),
            form_index: HashMap::new(),
            category_index: HashMap::new(),
            group_index: HashMap::new(),
            stats: VerbStats::default(),
        }
    }

    /// Create database with all built-in verbs
    pub fn with_builtin() -> Self {
        let mut db = Self::new();
        db.load_builtin_verbs();
        db.rebuild_indexes();
        db
    }

    /// Add a verb entry
    pub fn add(&mut self, entry: VerbEntry) {
        let base = entry.base.clone();

        // Add to form index
        self.form_index.insert(entry.base.clone(), base.clone());
        self.form_index.insert(entry.past.clone(), base.clone());
        self.form_index.insert(entry.past_participle.clone(), base.clone());
        self.form_index.insert(entry.present_participle.clone(), base.clone());
        self.form_index.insert(entry.third_person.clone(), base.clone());

        // Add to category index
        self.category_index
            .entry(entry.category)
            .or_default()
            .push(base.clone());

        // Add to group index
        self.group_index
            .entry(entry.group)
            .or_default()
            .push(base.clone());

        // Store entry
        self.verbs.insert(base, entry);
    }

    /// Look up a verb by any form
    pub fn lookup(&self, word: &str) -> Option<&VerbEntry> {
        let w = word.to_lowercase();
        self.form_index.get(&w)
            .and_then(|base| self.verbs.get(base))
    }

    /// Get all verbs in a category
    pub fn by_category(&self, category: FunctionalCategory) -> Vec<&VerbEntry> {
        self.category_index.get(&category)
            .map(|bases| bases.iter().filter_map(|b| self.verbs.get(b)).collect())
            .unwrap_or_default()
    }

    /// Get all verbs in a group
    pub fn by_group(&self, group: VerbGroup) -> Vec<&VerbEntry> {
        self.group_index.get(&group)
            .map(|bases| bases.iter().filter_map(|b| self.verbs.get(b)).collect())
            .unwrap_or_default()
    }

    /// Get the base form of any verb form
    pub fn base_form(&self, word: &str) -> Option<&str> {
        self.form_index.get(&word.to_lowercase()).map(|s| s.as_str())
    }

    /// Check if a word is a known verb form
    pub fn is_verb(&self, word: &str) -> bool {
        self.form_index.contains_key(&word.to_lowercase())
    }

    /// Get category of a verb
    pub fn get_category(&self, word: &str) -> Option<FunctionalCategory> {
        self.lookup(word).map(|e| e.category)
    }

    /// Get group of a verb
    pub fn get_group(&self, word: &str) -> Option<VerbGroup> {
        self.lookup(word).map(|e| e.group)
    }

    /// Rebuild all indexes and stats
    fn rebuild_indexes(&mut self) {
        let mut irregular = 0;
        let mut regular = 0;

        for entry in self.verbs.values() {
            if entry.irregular {
                irregular += 1;
            } else {
                regular += 1;
            }
        }

        self.stats = VerbStats {
            total_verbs: self.verbs.len(),
            irregular_verbs: irregular,
            regular_verbs: regular,
            total_forms: self.form_index.len(),
            categories_used: self.category_index.len(),
            groups_used: self.group_index.len(),
        };
    }

    /// Get all verbs as iterator
    pub fn all_verbs(&self) -> impl Iterator<Item = &VerbEntry> {
        self.verbs.values()
    }

    /// Get total count
    pub fn len(&self) -> usize {
        self.verbs.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.verbs.is_empty()
    }
}

impl Default for VerbDatabase {
    fn default() -> Self {
        Self::with_builtin()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regular_verb() {
        let v = VerbEntry::regular("walk", FunctionalCategory::Movement, VerbGroup::Walk, None, 90);
        assert_eq!(v.past, "walked");
        assert_eq!(v.present_participle, "walking");
        assert_eq!(v.third_person, "walks");
    }

    #[test]
    fn test_irregular_verb() {
        let v = VerbEntry::irregular("go", "went", "gone", FunctionalCategory::Movement, VerbGroup::Walk, None, 100);
        assert_eq!(v.past, "went");
        assert_eq!(v.past_participle, "gone");
        assert!(v.irregular);
    }

    #[test]
    fn test_database_lookup() {
        let db = VerbDatabase::with_builtin();
        assert!(db.is_verb("run"));
        assert!(db.is_verb("running"));
        assert!(db.is_verb("ran"));
        assert_eq!(db.base_form("running"), Some("run"));
    }

    #[test]
    fn test_category_lookup() {
        let db = VerbDatabase::with_builtin();
        let movement_verbs = db.by_category(FunctionalCategory::Movement);
        assert!(!movement_verbs.is_empty());
    }

    #[test]
    fn test_verb_count() {
        let db = VerbDatabase::with_builtin();
        println!("Total verbs: {}", db.stats.total_verbs);
        println!("Irregular: {}", db.stats.irregular_verbs);
        println!("Total forms indexed: {}", db.stats.total_forms);
        assert!(db.stats.total_verbs > 500);
    }
}
