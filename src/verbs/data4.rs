//! Complete English Verb Database - Part 4
//! Body, Consumption, Aspectual, Causation, Attempt, Connection,
//! Position, Measurement, Modal, Weather, Emission verbs

use super::{VerbDatabase, VerbEntry, FunctionalCategory, VerbGroup};

impl VerbDatabase {
    pub(super) fn load_consumption_verbs(&mut self) {
        use FunctionalCategory::Consumption;

        // EAT group
        self.add(VerbEntry::irregular("eat", "ate", "eaten", Consumption, VerbGroup::Eat, Some(true), 90)
            .with_synonyms(&["devour", "consume", "swallow", "munch", "chew", "bite", "gobble", "feast"]));
        self.add(VerbEntry::regular("devour", Consumption, VerbGroup::Eat, Some(true), 40));
        self.add(VerbEntry::regular("consume", Consumption, VerbGroup::Eat, Some(true), 55));
        self.add(VerbEntry::regular("swallow", Consumption, VerbGroup::Eat, Some(true), 50));
        self.add(VerbEntry::regular("munch", Consumption, VerbGroup::Eat, Some(true), 35));
        self.add(VerbEntry::regular("chew", Consumption, VerbGroup::Eat, Some(true), 50));
        self.add(VerbEntry::irregular("bite", "bit", "bitten", Consumption, VerbGroup::Eat, Some(true), 60));
        self.add(VerbEntry::regular("gobble", Consumption, VerbGroup::Eat, Some(true), 30));
        self.add(VerbEntry::regular("feast", Consumption, VerbGroup::Eat, None, 35));
        self.add(VerbEntry::regular("dine", Consumption, VerbGroup::Eat, None, 45));
        self.add(VerbEntry::regular("snack", Consumption, VerbGroup::Eat, None, 45));
        self.add(VerbEntry::regular("nibble", Consumption, VerbGroup::Eat, Some(true), 35));
        self.add(VerbEntry::regular("taste", Consumption, VerbGroup::Eat, Some(true), 60));
        self.add(VerbEntry::regular("lick", Consumption, VerbGroup::Eat, Some(true), 45));
        self.add(VerbEntry::regular("suck", Consumption, VerbGroup::Eat, Some(true), 45));
        self.add(VerbEntry::regular("digest", Consumption, VerbGroup::Eat, Some(true), 40));
        self.add(VerbEntry::regular("absorb", Consumption, VerbGroup::Eat, Some(true), 50));
        self.add(VerbEntry::regular("ingest", Consumption, VerbGroup::Eat, Some(true), 30));
        self.add(VerbEntry::regular("feed", Consumption, VerbGroup::Eat, Some(true), 65));
        self.add(VerbEntry::regular("nourish", Consumption, VerbGroup::Eat, Some(true), 35));
        self.add(VerbEntry::regular("starve", Consumption, VerbGroup::Eat, None, 40));
        self.add(VerbEntry::regular("fast", Consumption, VerbGroup::Eat, None, 35));

        // DRINK group
        self.add(VerbEntry::irregular("drink", "drank", "drunk", Consumption, VerbGroup::Drink, Some(true), 80)
            .with_synonyms(&["sip", "gulp", "swallow", "guzzle", "quaff", "slurp"]));
        self.add(VerbEntry::regular("sip", Consumption, VerbGroup::Drink, Some(true), 50));
        self.add(VerbEntry::regular("gulp", Consumption, VerbGroup::Drink, Some(true), 45));
        self.add(VerbEntry::regular("guzzle", Consumption, VerbGroup::Drink, Some(true), 30));
        self.add(VerbEntry::regular("quaff", Consumption, VerbGroup::Drink, Some(true), 20));
        self.add(VerbEntry::regular("slurp", Consumption, VerbGroup::Drink, Some(true), 30));
        self.add(VerbEntry::regular("imbibe", Consumption, VerbGroup::Drink, Some(true), 25));

        // BREATHE group (also consumption)
        self.add(VerbEntry::regular("breathe", Consumption, VerbGroup::Breathe, None, 65)
            .with_synonyms(&["inhale", "exhale", "respire", "pant", "gasp"]));
        self.add(VerbEntry::regular("inhale", Consumption, VerbGroup::Breathe, Some(true), 45));
        self.add(VerbEntry::regular("exhale", Consumption, VerbGroup::Breathe, Some(true), 45));
        self.add(VerbEntry::regular("respire", Consumption, VerbGroup::Breathe, None, 25));
        self.add(VerbEntry::regular("pant", Consumption, VerbGroup::Breathe, None, 35));
        self.add(VerbEntry::regular("gasp", Consumption, VerbGroup::Breathe, None, 40));
        self.add(VerbEntry::regular("sigh", Consumption, VerbGroup::Breathe, None, 45));
        self.add(VerbEntry::regular("yawn", Consumption, VerbGroup::Breathe, None, 40));
        self.add(VerbEntry::regular("cough", Consumption, VerbGroup::Breathe, None, 50));
        self.add(VerbEntry::regular("sneeze", Consumption, VerbGroup::Breathe, None, 40));
        self.add(VerbEntry::regular("hiccup", Consumption, VerbGroup::Breathe, None, 30));
        self.add(VerbEntry::regular("snore", Consumption, VerbGroup::Breathe, None, 35));
        self.add(VerbEntry::regular("wheeze", Consumption, VerbGroup::Breathe, None, 30));
        self.add(VerbEntry::regular("choke", Consumption, VerbGroup::Breathe, None, 45));
        self.add(VerbEntry::regular("suffocate", Consumption, VerbGroup::Breathe, None, 35));
    }

    pub(super) fn load_body_verbs(&mut self) {
        use FunctionalCategory::Body;

        // SLEEP group
        self.add(VerbEntry::irregular("sleep", "slept", "slept", Body, VerbGroup::Sleep, None, 80)
            .with_synonyms(&["nap", "doze", "rest", "slumber", "snooze"])
            .with_antonyms(&["wake", "awaken"]));
        self.add(VerbEntry::regular("nap", Body, VerbGroup::Sleep, None, 50));
        self.add(VerbEntry::regular("doze", Body, VerbGroup::Sleep, None, 35));
        self.add(VerbEntry::regular("rest", Body, VerbGroup::Sleep, None, 70));
        self.add(VerbEntry::regular("slumber", Body, VerbGroup::Sleep, None, 30));
        self.add(VerbEntry::regular("snooze", Body, VerbGroup::Sleep, None, 35));
        self.add(VerbEntry::regular("relax", Body, VerbGroup::Sleep, None, 65));
        self.add(VerbEntry::regular("dream", Body, VerbGroup::Sleep, None, 60));

        // WAKE group
        self.add(VerbEntry::irregular("wake", "woke", "woken", Body, VerbGroup::Wake, None, 70)
            .with_synonyms(&["awaken", "arise", "stir", "rouse"])
            .with_antonyms(&["sleep"]));
        self.add(VerbEntry::regular("awaken", Body, VerbGroup::Wake, None, 45));
        self.add(VerbEntry::irregular("arise", "arose", "arisen", Body, VerbGroup::Wake, None, 40));
        self.add(VerbEntry::regular("stir", Body, VerbGroup::Wake, None, 45));
        self.add(VerbEntry::regular("rouse", Body, VerbGroup::Wake, Some(true), 30));

        // SIT group
        self.add(VerbEntry::irregular("sit", "sat", "sat", Body, VerbGroup::Sit, None, 85)
            .with_synonyms(&["seat", "perch", "settle"]));
        self.add(VerbEntry::regular("seat", Body, VerbGroup::Sit, Some(true), 50));
        self.add(VerbEntry::regular("perch", Body, VerbGroup::Sit, None, 35));
        self.add(VerbEntry::regular("settle", Body, VerbGroup::Sit, None, 55));
        self.add(VerbEntry::regular("squat", Body, VerbGroup::Sit, None, 35));
        self.add(VerbEntry::regular("crouch", Body, VerbGroup::Sit, None, 40));

        // STAND group
        self.add(VerbEntry::irregular("stand", "stood", "stood", Body, VerbGroup::Stand, None, 85)
            .with_synonyms(&["rise", "arise", "get up"])
            .with_antonyms(&["sit", "lie"]));
        self.add(VerbEntry::irregular("rise", "rose", "risen", Body, VerbGroup::Stand, None, 70));
        self.add(VerbEntry::regular("tower", Body, VerbGroup::Stand, None, 40));
        self.add(VerbEntry::regular("loom", Body, VerbGroup::Stand, None, 35));

        // LIE group
        self.add(VerbEntry::irregular("lie", "lay", "lain", Body, VerbGroup::Lie, None, 65)
            .with_synonyms(&["recline", "stretch", "sprawl"]));
        self.add(VerbEntry::regular("recline", Body, VerbGroup::Lie, None, 35));
        self.add(VerbEntry::regular("stretch", Body, VerbGroup::Lie, None, 55));
        self.add(VerbEntry::regular("sprawl", Body, VerbGroup::Lie, None, 30));
        self.add(VerbEntry::irregular("lay", "laid", "laid", Body, VerbGroup::Lie, Some(true), 65));
        self.add(VerbEntry::regular("lounge", Body, VerbGroup::Lie, None, 35));

        // KNEEL group
        self.add(VerbEntry::irregular("kneel", "knelt", "knelt", Body, VerbGroup::Kneel, None, 45)
            .with_synonyms(&["crouch", "bow", "stoop"]));
        self.add(VerbEntry::regular("bow", Body, VerbGroup::Kneel, None, 50));
        self.add(VerbEntry::regular("stoop", Body, VerbGroup::Kneel, None, 35));
        self.add(VerbEntry::regular("curtsy", Body, VerbGroup::Kneel, None, 25));

        // BEND group
        self.add(VerbEntry::irregular("bend", "bent", "bent", Body, VerbGroup::Bend, None, 60)
            .with_synonyms(&["lean", "tilt", "incline", "slouch"]));
        self.add(VerbEntry::regular("lean", Body, VerbGroup::Bend, None, 55));
        self.add(VerbEntry::regular("tilt", Body, VerbGroup::Bend, Some(true), 45));
        self.add(VerbEntry::regular("incline", Body, VerbGroup::Bend, None, 35));
        self.add(VerbEntry::regular("slouch", Body, VerbGroup::Bend, None, 30));
        self.add(VerbEntry::regular("hunch", Body, VerbGroup::Bend, None, 30));
        self.add(VerbEntry::regular("flex", Body, VerbGroup::Bend, Some(true), 40));

        // Additional body function verbs
        self.add(VerbEntry::regular("blink", Body, VerbGroup::Wake, None, 45));
        self.add(VerbEntry::regular("wink", Body, VerbGroup::Wake, None, 40));
        self.add(VerbEntry::regular("frown", Body, VerbGroup::Bend, None, 40));
        self.add(VerbEntry::regular("smile", Body, VerbGroup::Wake, None, 65));
        self.add(VerbEntry::regular("laugh", Body, VerbGroup::Wake, None, 70));
        self.add(VerbEntry::regular("cry", Body, VerbGroup::Sleep, None, 65));
        self.add(VerbEntry::irregular("weep", "wept", "wept", Body, VerbGroup::Sleep, None, 35));
        self.add(VerbEntry::regular("shiver", Body, VerbGroup::Bend, None, 40));
        self.add(VerbEntry::regular("tremble", Body, VerbGroup::Bend, None, 40));
        self.add(VerbEntry::irregular("shake", "shook", "shaken", Body, VerbGroup::Bend, None, 60));
        self.add(VerbEntry::regular("sweat", Body, VerbGroup::Wake, None, 45));
        self.add(VerbEntry::regular("blush", Body, VerbGroup::Wake, None, 40));
        self.add(VerbEntry::regular("faint", Body, VerbGroup::Sleep, None, 40));
        self.add(VerbEntry::regular("vomit", Body, VerbGroup::Wake, None, 35));
        self.add(VerbEntry::irregular("bleed", "bled", "bled", Body, VerbGroup::Wake, None, 45));
        self.add(VerbEntry::regular("heal", Body, VerbGroup::Wake, None, 55));
        self.add(VerbEntry::regular("recover", Body, VerbGroup::Wake, None, 55));
        self.add(VerbEntry::irregular("grow", "grew", "grown", Body, VerbGroup::Wake, None, 70));
        self.add(VerbEntry::regular("age", Body, VerbGroup::Wake, None, 45));
        self.add(VerbEntry::irregular("bear", "bore", "born", Body, VerbGroup::Wake, Some(true), 50));
        self.add(VerbEntry::regular("die", Body, VerbGroup::Sleep, None, 70));
    }

    pub(super) fn load_aspectual_verbs(&mut self) {
        use FunctionalCategory::Aspectual;

        // BEGIN group
        self.add(VerbEntry::irregular("begin", "began", "begun", Aspectual, VerbGroup::Begin, Some(true), 85)
            .with_synonyms(&["start", "commence", "initiate", "launch", "open", "originate"])
            .with_antonyms(&["end", "finish", "stop"]));
        self.add(VerbEntry::regular("start", Aspectual, VerbGroup::Begin, Some(true), 95));
        self.add(VerbEntry::regular("commence", Aspectual, VerbGroup::Begin, Some(true), 40));
        self.add(VerbEntry::regular("initiate", Aspectual, VerbGroup::Begin, Some(true), 45));
        self.add(VerbEntry::regular("launch", Aspectual, VerbGroup::Begin, Some(true), 55));
        self.add(VerbEntry::regular("open", Aspectual, VerbGroup::Begin, Some(true), 80));
        self.add(VerbEntry::regular("originate", Aspectual, VerbGroup::Begin, None, 35));
        self.add(VerbEntry::regular("introduce", Aspectual, VerbGroup::Begin, Some(true), 60));
        self.add(VerbEntry::regular("trigger", Aspectual, VerbGroup::Begin, Some(true), 50));
        self.add(VerbEntry::regular("activate", Aspectual, VerbGroup::Begin, Some(true), 45));
        self.add(VerbEntry::regular("spark", Aspectual, VerbGroup::Begin, Some(true), 45));

        // END group
        self.add(VerbEntry::regular("end", Aspectual, VerbGroup::End, Some(true), 85)
            .with_synonyms(&["finish", "complete", "conclude", "terminate", "close", "cease"])
            .with_antonyms(&["begin", "start"]));
        self.add(VerbEntry::regular("finish", Aspectual, VerbGroup::End, Some(true), 85));
        self.add(VerbEntry::regular("complete", Aspectual, VerbGroup::End, Some(true), 75));
        self.add(VerbEntry::regular("conclude", Aspectual, VerbGroup::End, Some(true), 50));
        self.add(VerbEntry::regular("terminate", Aspectual, VerbGroup::End, Some(true), 45));
        self.add(VerbEntry::regular("close", Aspectual, VerbGroup::End, Some(true), 75));
        self.add(VerbEntry::regular("cease", Aspectual, VerbGroup::End, None, 40));
        self.add(VerbEntry::regular("expire", Aspectual, VerbGroup::End, None, 40));
        self.add(VerbEntry::regular("accomplish", Aspectual, VerbGroup::End, Some(true), 50));
        self.add(VerbEntry::regular("achieve", Aspectual, VerbGroup::End, Some(true), 65));
        self.add(VerbEntry::regular("fulfill", Aspectual, VerbGroup::End, Some(true), 50));
        self.add(VerbEntry::regular("wrap", Aspectual, VerbGroup::End, Some(true), 50));

        // CONTINUE group
        self.add(VerbEntry::regular("continue", Aspectual, VerbGroup::Continue, Some(true), 80)
            .with_synonyms(&["proceed", "persist", "persevere", "carry on", "go on", "maintain"]));
        self.add(VerbEntry::regular("proceed", Aspectual, VerbGroup::Continue, None, 55));
        self.add(VerbEntry::regular("persist", Aspectual, VerbGroup::Continue, None, 45));
        self.add(VerbEntry::regular("persevere", Aspectual, VerbGroup::Continue, None, 35));
        self.add(VerbEntry::regular("maintain", Aspectual, VerbGroup::Continue, Some(true), 60));
        self.add(VerbEntry::regular("sustain", Aspectual, VerbGroup::Continue, Some(true), 45));
        self.add(VerbEntry::regular("prolong", Aspectual, VerbGroup::Continue, Some(true), 35));
        self.add(VerbEntry::regular("extend", Aspectual, VerbGroup::Continue, Some(true), 55));
        self.add(VerbEntry::regular("resume", Aspectual, VerbGroup::Continue, Some(true), 50));

        // STOP group
        self.add(VerbEntry::regular("stop", Aspectual, VerbGroup::Stop, Some(true), 90)
            .with_synonyms(&["halt", "cease", "pause", "discontinue", "quit", "suspend"]));
        self.add(VerbEntry::regular("halt", Aspectual, VerbGroup::Stop, Some(true), 50));
        self.add(VerbEntry::regular("pause", Aspectual, VerbGroup::Stop, None, 55));
        self.add(VerbEntry::regular("discontinue", Aspectual, VerbGroup::Stop, Some(true), 35));
        self.add(VerbEntry::irregular("quit", "quit", "quit", Aspectual, VerbGroup::Stop, Some(true), 65));
        self.add(VerbEntry::regular("suspend", Aspectual, VerbGroup::Stop, Some(true), 45));
        self.add(VerbEntry::regular("interrupt", Aspectual, VerbGroup::Stop, Some(true), 50));
        self.add(VerbEntry::regular("freeze", Aspectual, VerbGroup::Stop, Some(true), 50));

        // REPEAT group
        self.add(VerbEntry::regular("repeat", Aspectual, VerbGroup::Repeat, Some(true), 65)
            .with_synonyms(&["redo", "reiterate", "recur", "replay", "rerun"]));
        self.add(VerbEntry::regular("redo", Aspectual, VerbGroup::Repeat, Some(true), 45));
        self.add(VerbEntry::regular("reiterate", Aspectual, VerbGroup::Repeat, Some(true), 35));
        self.add(VerbEntry::regular("recur", Aspectual, VerbGroup::Repeat, None, 35));
        self.add(VerbEntry::regular("replay", Aspectual, VerbGroup::Repeat, Some(true), 40));
        self.add(VerbEntry::regular("rerun", Aspectual, VerbGroup::Repeat, Some(true), 35));
        self.add(VerbEntry::regular("renew", Aspectual, VerbGroup::Repeat, Some(true), 50));
        self.add(VerbEntry::regular("restore", Aspectual, VerbGroup::Repeat, Some(true), 50));
        self.add(VerbEntry::regular("revive", Aspectual, VerbGroup::Repeat, Some(true), 40));
        self.add(VerbEntry::regular("resurrect", Aspectual, VerbGroup::Repeat, Some(true), 30));
    }

    pub(super) fn load_causation_verbs(&mut self) {
        use FunctionalCategory::Causation;

        // CAUSE group
        self.add(VerbEntry::regular("cause", Causation, VerbGroup::Cause, Some(true), 75)
            .with_synonyms(&["make", "produce", "create", "bring about", "trigger", "induce"]));
        self.add(VerbEntry::regular("produce", Causation, VerbGroup::Cause, Some(true), 70));
        self.add(VerbEntry::regular("create", Causation, VerbGroup::Cause, Some(true), 75));
        self.add(VerbEntry::regular("trigger", Causation, VerbGroup::Cause, Some(true), 50));
        self.add(VerbEntry::regular("induce", Causation, VerbGroup::Cause, Some(true), 40));
        self.add(VerbEntry::regular("generate", Causation, VerbGroup::Cause, Some(true), 55));
        self.add(VerbEntry::regular("provoke", Causation, VerbGroup::Cause, Some(true), 45));
        self.add(VerbEntry::regular("spark", Causation, VerbGroup::Cause, Some(true), 45));
        self.add(VerbEntry::regular("prompt", Causation, VerbGroup::Cause, Some(true), 45));
        self.add(VerbEntry::regular("stimulate", Causation, VerbGroup::Cause, Some(true), 40));
        self.add(VerbEntry::regular("motivate", Causation, VerbGroup::Cause, Some(true), 50));
        self.add(VerbEntry::regular("inspire", Causation, VerbGroup::Cause, Some(true), 55));

        // ALLOW group
        self.add(VerbEntry::regular("allow", Causation, VerbGroup::Allow, Some(true), 80)
            .with_synonyms(&["permit", "let", "enable", "authorize", "empower"])
            .with_antonyms(&["prevent", "forbid", "prohibit"]));
        self.add(VerbEntry::regular("permit", Causation, VerbGroup::Allow, Some(true), 55));
        self.add(VerbEntry::irregular("let", "let", "let", Causation, VerbGroup::Allow, Some(true), 90));
        self.add(VerbEntry::regular("enable", Causation, VerbGroup::Allow, Some(true), 55));
        self.add(VerbEntry::regular("authorize", Causation, VerbGroup::Allow, Some(true), 45));
        self.add(VerbEntry::regular("empower", Causation, VerbGroup::Allow, Some(true), 40));
        self.add(VerbEntry::regular("sanction", Causation, VerbGroup::Allow, Some(true), 35));
        self.add(VerbEntry::regular("approve", Causation, VerbGroup::Allow, Some(true), 60));
        self.add(VerbEntry::regular("grant", Causation, VerbGroup::Allow, Some(true), 50));

        // PREVENT group
        self.add(VerbEntry::regular("prevent", Causation, VerbGroup::Prevent, Some(true), 70)
            .with_synonyms(&["stop", "block", "hinder", "impede", "obstruct", "prohibit"])
            .with_antonyms(&["allow", "permit", "enable"]));
        self.add(VerbEntry::regular("stop", Causation, VerbGroup::Prevent, Some(true), 85));
        self.add(VerbEntry::regular("block", Causation, VerbGroup::Prevent, Some(true), 60));
        self.add(VerbEntry::regular("hinder", Causation, VerbGroup::Prevent, Some(true), 40));
        self.add(VerbEntry::regular("impede", Causation, VerbGroup::Prevent, Some(true), 35));
        self.add(VerbEntry::regular("obstruct", Causation, VerbGroup::Prevent, Some(true), 40));
        self.add(VerbEntry::regular("prohibit", Causation, VerbGroup::Prevent, Some(true), 45));
        self.add(VerbEntry::irregular("forbid", "forbade", "forbidden", Causation, VerbGroup::Prevent, Some(true), 45));
        self.add(VerbEntry::regular("ban", Causation, VerbGroup::Prevent, Some(true), 50));
        self.add(VerbEntry::regular("restrict", Causation, VerbGroup::Prevent, Some(true), 50));
        self.add(VerbEntry::regular("limit", Causation, VerbGroup::Prevent, Some(true), 60));
        self.add(VerbEntry::regular("restrain", Causation, VerbGroup::Prevent, Some(true), 45));
        self.add(VerbEntry::regular("inhibit", Causation, VerbGroup::Prevent, Some(true), 40));
        self.add(VerbEntry::regular("suppress", Causation, VerbGroup::Prevent, Some(true), 45));
        self.add(VerbEntry::regular("deter", Causation, VerbGroup::Prevent, Some(true), 40));
        self.add(VerbEntry::regular("discourage", Causation, VerbGroup::Prevent, Some(true), 50));

        // FORCE group
        self.add(VerbEntry::regular("force", Causation, VerbGroup::Force, Some(true), 70)
            .with_synonyms(&["compel", "oblige", "coerce", "pressure", "drive", "push"]));
        self.add(VerbEntry::regular("compel", Causation, VerbGroup::Force, Some(true), 40));
        self.add(VerbEntry::regular("oblige", Causation, VerbGroup::Force, Some(true), 40));
        self.add(VerbEntry::regular("coerce", Causation, VerbGroup::Force, Some(true), 35));
        self.add(VerbEntry::regular("pressure", Causation, VerbGroup::Force, Some(true), 50));
        self.add(VerbEntry::regular("drive", Causation, VerbGroup::Force, Some(true), 70));
        self.add(VerbEntry::regular("push", Causation, VerbGroup::Force, Some(true), 70));
        self.add(VerbEntry::regular("urge", Causation, VerbGroup::Force, Some(true), 50));
        self.add(VerbEntry::regular("impel", Causation, VerbGroup::Force, Some(true), 25));
        self.add(VerbEntry::regular("require", Causation, VerbGroup::Force, Some(true), 65));
        self.add(VerbEntry::regular("demand", Causation, VerbGroup::Force, Some(true), 60));
        self.add(VerbEntry::regular("impose", Causation, VerbGroup::Force, Some(true), 45));

        // HELP (causative) group
        self.add(VerbEntry::regular("help", Causation, VerbGroup::HelpCausation, Some(true), 90));
        self.add(VerbEntry::regular("assist", Causation, VerbGroup::HelpCausation, Some(true), 55));
        self.add(VerbEntry::regular("aid", Causation, VerbGroup::HelpCausation, Some(true), 50));
        self.add(VerbEntry::regular("facilitate", Causation, VerbGroup::HelpCausation, Some(true), 45));
        self.add(VerbEntry::regular("support", Causation, VerbGroup::HelpCausation, Some(true), 70));
        self.add(VerbEntry::regular("encourage", Causation, VerbGroup::HelpCausation, Some(true), 60));
        self.add(VerbEntry::regular("promote", Causation, VerbGroup::HelpCausation, Some(true), 55));
        self.add(VerbEntry::regular("foster", Causation, VerbGroup::HelpCausation, Some(true), 40));
        self.add(VerbEntry::regular("boost", Causation, VerbGroup::HelpCausation, Some(true), 50));
        self.add(VerbEntry::regular("enhance", Causation, VerbGroup::HelpCausation, Some(true), 50));
    }

    pub(super) fn load_attempt_verbs(&mut self) {
        use FunctionalCategory::Attempt;

        // TRY group
        self.add(VerbEntry::regular("try", Attempt, VerbGroup::Try, Some(true), 90)
            .with_synonyms(&["attempt", "endeavor", "strive", "seek", "aim"]));
        self.add(VerbEntry::regular("attempt", Attempt, VerbGroup::Try, Some(true), 60));
        self.add(VerbEntry::regular("endeavor", Attempt, VerbGroup::Try, None, 35));
        self.add(VerbEntry::irregular("strive", "strove", "striven", Attempt, VerbGroup::Try, None, 40));
        self.add(VerbEntry::irregular("seek", "sought", "sought", Attempt, VerbGroup::Try, Some(true), 60));
        self.add(VerbEntry::regular("aim", Attempt, VerbGroup::Try, None, 60));
        self.add(VerbEntry::regular("venture", Attempt, VerbGroup::Try, Some(true), 40));
        self.add(VerbEntry::regular("dare", Attempt, VerbGroup::Try, Some(true), 50));
        self.add(VerbEntry::regular("risk", Attempt, VerbGroup::Try, Some(true), 55));
        self.add(VerbEntry::regular("struggle", Attempt, VerbGroup::Try, None, 55));
        self.add(VerbEntry::regular("fight", Attempt, VerbGroup::Try, None, 65));

        // SUCCEED group
        self.add(VerbEntry::regular("succeed", Attempt, VerbGroup::Succeed, None, 70)
            .with_synonyms(&["accomplish", "achieve", "attain", "manage", "prevail"])
            .with_antonyms(&["fail"]));
        self.add(VerbEntry::regular("accomplish", Attempt, VerbGroup::Succeed, Some(true), 55));
        self.add(VerbEntry::regular("achieve", Attempt, VerbGroup::Succeed, Some(true), 65));
        self.add(VerbEntry::regular("attain", Attempt, VerbGroup::Succeed, Some(true), 40));
        self.add(VerbEntry::regular("manage", Attempt, VerbGroup::Succeed, Some(true), 70));
        self.add(VerbEntry::regular("prevail", Attempt, VerbGroup::Succeed, None, 40));
        self.add(VerbEntry::regular("triumph", Attempt, VerbGroup::Succeed, None, 40));
        self.add(VerbEntry::irregular("overcome", "overcame", "overcome", Attempt, VerbGroup::Succeed, Some(true), 55));
        self.add(VerbEntry::regular("conquer", Attempt, VerbGroup::Succeed, Some(true), 45));
        self.add(VerbEntry::regular("master", Attempt, VerbGroup::Succeed, Some(true), 50));
        self.add(VerbEntry::regular("prosper", Attempt, VerbGroup::Succeed, None, 40));
        self.add(VerbEntry::regular("thrive", Attempt, VerbGroup::Succeed, None, 45));
        self.add(VerbEntry::regular("flourish", Attempt, VerbGroup::Succeed, None, 40));

        // FAIL group
        self.add(VerbEntry::regular("fail", Attempt, VerbGroup::Fail, None, 70)
            .with_synonyms(&["flunk", "miss", "lose", "flounder", "collapse"])
            .with_antonyms(&["succeed"]));
        self.add(VerbEntry::regular("flunk", Attempt, VerbGroup::Fail, Some(true), 35));
        self.add(VerbEntry::regular("miss", Attempt, VerbGroup::Fail, Some(true), 75));
        self.add(VerbEntry::irregular("lose", "lost", "lost", Attempt, VerbGroup::Fail, Some(true), 80));
        self.add(VerbEntry::regular("flounder", Attempt, VerbGroup::Fail, None, 30));
        self.add(VerbEntry::regular("collapse", Attempt, VerbGroup::Fail, None, 55));
        self.add(VerbEntry::regular("falter", Attempt, VerbGroup::Fail, None, 35));
        self.add(VerbEntry::regular("stumble", Attempt, VerbGroup::Fail, None, 45));
        self.add(VerbEntry::regular("bomb", Attempt, VerbGroup::Fail, None, 40));
        self.add(VerbEntry::regular("botch", Attempt, VerbGroup::Fail, Some(true), 30));
        self.add(VerbEntry::regular("bungle", Attempt, VerbGroup::Fail, Some(true), 25));

        // PRACTICE group
        self.add(VerbEntry::regular("practice", Attempt, VerbGroup::Practice, Some(true), 65)
            .with_synonyms(&["rehearse", "train", "drill", "exercise", "prepare"]));
        self.add(VerbEntry::regular("rehearse", Attempt, VerbGroup::Practice, Some(true), 45));
        self.add(VerbEntry::regular("train", Attempt, VerbGroup::Practice, None, 65));
        self.add(VerbEntry::regular("drill", Attempt, VerbGroup::Practice, Some(true), 40));
        self.add(VerbEntry::regular("exercise", Attempt, VerbGroup::Practice, Some(true), 60));
        self.add(VerbEntry::regular("prepare", Attempt, VerbGroup::Practice, Some(true), 70));
        self.add(VerbEntry::regular("warm up", Attempt, VerbGroup::Practice, None, 50));
    }

    pub(super) fn load_connection_verbs(&mut self) {
        use FunctionalCategory::Connection;

        // CONNECT group
        self.add(VerbEntry::regular("connect", Connection, VerbGroup::Connect, Some(true), 70)
            .with_synonyms(&["join", "link", "attach", "unite", "couple", "bond"])
            .with_antonyms(&["disconnect", "separate"]));
        self.add(VerbEntry::regular("join", Connection, VerbGroup::Connect, Some(true), 75));
        self.add(VerbEntry::regular("link", Connection, VerbGroup::Connect, Some(true), 60));
        self.add(VerbEntry::regular("attach", Connection, VerbGroup::Connect, Some(true), 60));
        self.add(VerbEntry::regular("unite", Connection, VerbGroup::Connect, Some(true), 50));
        self.add(VerbEntry::regular("couple", Connection, VerbGroup::Connect, Some(true), 40));
        self.add(VerbEntry::regular("bond", Connection, VerbGroup::Connect, Some(true), 45));
        self.add(VerbEntry::regular("bind", Connection, VerbGroup::Connect, Some(true), 50));
        self.add(VerbEntry::regular("tie", Connection, VerbGroup::Connect, Some(true), 60));
        self.add(VerbEntry::regular("fasten", Connection, VerbGroup::Connect, Some(true), 45));
        self.add(VerbEntry::regular("secure", Connection, VerbGroup::Connect, Some(true), 55));
        self.add(VerbEntry::regular("anchor", Connection, VerbGroup::Connect, Some(true), 40));
        self.add(VerbEntry::regular("hook", Connection, VerbGroup::Connect, Some(true), 45));
        self.add(VerbEntry::regular("clasp", Connection, VerbGroup::Connect, Some(true), 35));
        self.add(VerbEntry::regular("glue", Connection, VerbGroup::Connect, Some(true), 45));
        self.add(VerbEntry::irregular("stick", "stuck", "stuck", Connection, VerbGroup::Connect, Some(true), 60));
        self.add(VerbEntry::regular("tape", Connection, VerbGroup::Connect, Some(true), 45));
        self.add(VerbEntry::regular("pin", Connection, VerbGroup::Connect, Some(true), 45));
        self.add(VerbEntry::regular("nail", Connection, VerbGroup::Connect, Some(true), 45));
        self.add(VerbEntry::regular("screw", Connection, VerbGroup::Connect, Some(true), 45));
        self.add(VerbEntry::regular("bolt", Connection, VerbGroup::Connect, Some(true), 40));
        self.add(VerbEntry::regular("weld", Connection, VerbGroup::Connect, Some(true), 35));

        // SEPARATE group
        self.add(VerbEntry::regular("separate", Connection, VerbGroup::Separate, Some(true), 65)
            .with_synonyms(&["divide", "split", "part", "disconnect", "detach", "sever"])
            .with_antonyms(&["connect", "join", "unite"]));
        self.add(VerbEntry::regular("divide", Connection, VerbGroup::Separate, Some(true), 60));
        self.add(VerbEntry::irregular("split", "split", "split", Connection, VerbGroup::Separate, Some(true), 55));
        self.add(VerbEntry::regular("part", Connection, VerbGroup::Separate, Some(true), 50));
        self.add(VerbEntry::regular("disconnect", Connection, VerbGroup::Separate, Some(true), 45));
        self.add(VerbEntry::regular("detach", Connection, VerbGroup::Separate, Some(true), 45));
        self.add(VerbEntry::regular("sever", Connection, VerbGroup::Separate, Some(true), 35));
        self.add(VerbEntry::regular("isolate", Connection, VerbGroup::Separate, Some(true), 45));
        self.add(VerbEntry::regular("segregate", Connection, VerbGroup::Separate, Some(true), 35));
        self.add(VerbEntry::regular("remove", Connection, VerbGroup::Separate, Some(true), 75));
        self.add(VerbEntry::regular("extract", Connection, VerbGroup::Separate, Some(true), 45));
        self.add(VerbEntry::regular("untie", Connection, VerbGroup::Separate, Some(true), 40));
        self.add(VerbEntry::regular("unfasten", Connection, VerbGroup::Separate, Some(true), 35));
        self.add(VerbEntry::regular("unplug", Connection, VerbGroup::Separate, Some(true), 40));
        self.add(VerbEntry::regular("disengage", Connection, VerbGroup::Separate, Some(true), 35));
        self.add(VerbEntry::regular("release", Connection, VerbGroup::Separate, Some(true), 60));
        self.add(VerbEntry::regular("loosen", Connection, VerbGroup::Separate, Some(true), 45));
        self.add(VerbEntry::irregular("undo", "undid", "undone", Connection, VerbGroup::Separate, Some(true), 50));

        // COMBINE group
        self.add(VerbEntry::regular("combine", Connection, VerbGroup::Combine, Some(true), 60)
            .with_synonyms(&["merge", "mix", "blend", "fuse", "integrate", "unify"]));
        self.add(VerbEntry::regular("merge", Connection, VerbGroup::Combine, Some(true), 50));
        self.add(VerbEntry::regular("mix", Connection, VerbGroup::Combine, Some(true), 65));
        self.add(VerbEntry::regular("blend", Connection, VerbGroup::Combine, Some(true), 50));
        self.add(VerbEntry::regular("fuse", Connection, VerbGroup::Combine, Some(true), 40));
        self.add(VerbEntry::regular("integrate", Connection, VerbGroup::Combine, Some(true), 50));
        self.add(VerbEntry::regular("unify", Connection, VerbGroup::Combine, Some(true), 40));
        self.add(VerbEntry::regular("consolidate", Connection, VerbGroup::Combine, Some(true), 40));
        self.add(VerbEntry::regular("incorporate", Connection, VerbGroup::Combine, Some(true), 45));
        self.add(VerbEntry::regular("assimilate", Connection, VerbGroup::Combine, Some(true), 35));
        self.add(VerbEntry::regular("synthesize", Connection, VerbGroup::Combine, Some(true), 35));
        self.add(VerbEntry::regular("mingle", Connection, VerbGroup::Combine, None, 35));
        self.add(VerbEntry::regular("intermingle", Connection, VerbGroup::Combine, None, 25));
        self.add(VerbEntry::regular("pool", Connection, VerbGroup::Combine, Some(true), 40));

        // ATTACH group (physical connection)
        self.add(VerbEntry::regular("clip", Connection, VerbGroup::Attach, Some(true), 45));
        self.add(VerbEntry::regular("staple", Connection, VerbGroup::Attach, Some(true), 40));
        self.add(VerbEntry::regular("clamp", Connection, VerbGroup::Attach, Some(true), 40));
        self.add(VerbEntry::irregular("sew", "sewed", "sewn", Connection, VerbGroup::Attach, Some(true), 45));
        self.add(VerbEntry::irregular("weave", "wove", "woven", Connection, VerbGroup::Attach, Some(true), 40));
        self.add(VerbEntry::irregular("knit", "knit", "knit", Connection, VerbGroup::Attach, Some(true), 40));
        self.add(VerbEntry::regular("braid", Connection, VerbGroup::Attach, Some(true), 30));
        self.add(VerbEntry::regular("interlock", Connection, VerbGroup::Attach, Some(true), 30));
        self.add(VerbEntry::regular("intertwine", Connection, VerbGroup::Attach, Some(true), 30));
    }

    pub(super) fn load_position_verbs(&mut self) {
        use FunctionalCategory::Position;

        // PUT group
        self.add(VerbEntry::irregular("put", "put", "put", Position, VerbGroup::Put, Some(true), 95)
            .with_synonyms(&["place", "set", "lay", "position", "deposit", "locate"]));
        self.add(VerbEntry::regular("place", Position, VerbGroup::Put, Some(true), 75));
        self.add(VerbEntry::irregular("set", "set", "set", Position, VerbGroup::Put, Some(true), 90));
        self.add(VerbEntry::irregular("lay", "laid", "laid", Position, VerbGroup::Put, Some(true), 65));
        self.add(VerbEntry::regular("position", Position, VerbGroup::Put, Some(true), 50));
        self.add(VerbEntry::regular("deposit", Position, VerbGroup::Put, Some(true), 45));
        self.add(VerbEntry::regular("locate", Position, VerbGroup::Put, Some(true), 50));
        self.add(VerbEntry::regular("situate", Position, VerbGroup::Put, Some(true), 35));
        self.add(VerbEntry::regular("station", Position, VerbGroup::Put, Some(true), 40));
        self.add(VerbEntry::regular("post", Position, VerbGroup::Put, Some(true), 50));
        self.add(VerbEntry::regular("park", Position, VerbGroup::Put, Some(true), 55));
        self.add(VerbEntry::regular("arrange", Position, VerbGroup::Put, Some(true), 60));
        self.add(VerbEntry::regular("organize", Position, VerbGroup::Put, Some(true), 60));
        self.add(VerbEntry::regular("align", Position, VerbGroup::Put, Some(true), 45));
        self.add(VerbEntry::regular("orient", Position, VerbGroup::Put, Some(true), 40));
        self.add(VerbEntry::regular("center", Position, VerbGroup::Put, Some(true), 45));

        // REMOVE group
        self.add(VerbEntry::regular("remove", Position, VerbGroup::Remove, Some(true), 80)
            .with_synonyms(&["take away", "extract", "withdraw", "eliminate", "clear"]));
        self.add(VerbEntry::regular("extract", Position, VerbGroup::Remove, Some(true), 50));
        self.add(VerbEntry::irregular("withdraw", "withdrew", "withdrawn", Position, VerbGroup::Remove, Some(true), 50));
        self.add(VerbEntry::regular("eliminate", Position, VerbGroup::Remove, Some(true), 55));
        self.add(VerbEntry::regular("clear", Position, VerbGroup::Remove, Some(true), 65));
        self.add(VerbEntry::regular("empty", Position, VerbGroup::Remove, Some(true), 50));
        self.add(VerbEntry::regular("vacate", Position, VerbGroup::Remove, Some(true), 35));
        self.add(VerbEntry::regular("evacuate", Position, VerbGroup::Remove, Some(true), 40));
        self.add(VerbEntry::regular("relocate", Position, VerbGroup::Remove, Some(true), 40));
        self.add(VerbEntry::regular("shift", Position, VerbGroup::Remove, Some(true), 55));
        self.add(VerbEntry::regular("move", Position, VerbGroup::Remove, Some(true), 90));
        self.add(VerbEntry::regular("transfer", Position, VerbGroup::Remove, Some(true), 55));
        self.add(VerbEntry::regular("transport", Position, VerbGroup::Remove, Some(true), 50));
    }

    pub(super) fn load_measurement_verbs(&mut self) {
        use FunctionalCategory::Measurement;

        // MEASURE group
        self.add(VerbEntry::regular("measure", Measurement, VerbGroup::Measure, Some(true), 65)
            .with_synonyms(&["gauge", "quantify", "assess", "evaluate", "calculate"]));
        self.add(VerbEntry::regular("gauge", Measurement, VerbGroup::Measure, Some(true), 40));
        self.add(VerbEntry::regular("quantify", Measurement, VerbGroup::Measure, Some(true), 35));
        self.add(VerbEntry::regular("assess", Measurement, VerbGroup::Measure, Some(true), 55));
        self.add(VerbEntry::regular("evaluate", Measurement, VerbGroup::Measure, Some(true), 55));
        self.add(VerbEntry::regular("calculate", Measurement, VerbGroup::Measure, Some(true), 55));
        self.add(VerbEntry::regular("compute", Measurement, VerbGroup::Measure, Some(true), 45));
        self.add(VerbEntry::regular("determine", Measurement, VerbGroup::Measure, Some(true), 60));
        self.add(VerbEntry::regular("estimate", Measurement, VerbGroup::Measure, Some(true), 55));
        self.add(VerbEntry::regular("approximate", Measurement, VerbGroup::Measure, Some(true), 35));
        self.add(VerbEntry::regular("survey", Measurement, VerbGroup::Measure, Some(true), 45));
        self.add(VerbEntry::regular("rate", Measurement, VerbGroup::Measure, Some(true), 55));
        self.add(VerbEntry::regular("rank", Measurement, VerbGroup::Measure, Some(true), 50));
        self.add(VerbEntry::regular("grade", Measurement, VerbGroup::Measure, Some(true), 50));
        self.add(VerbEntry::regular("score", Measurement, VerbGroup::Measure, Some(true), 55));
        self.add(VerbEntry::regular("weigh", Measurement, VerbGroup::Measure, Some(true), 55));

        // COMPARE group
        self.add(VerbEntry::regular("compare", Measurement, VerbGroup::Compare, Some(true), 65)
            .with_synonyms(&["contrast", "match", "correlate", "relate", "equate"]));
        self.add(VerbEntry::regular("contrast", Measurement, VerbGroup::Compare, Some(true), 50));
        self.add(VerbEntry::regular("match", Measurement, VerbGroup::Compare, Some(true), 65));
        self.add(VerbEntry::regular("correlate", Measurement, VerbGroup::Compare, Some(true), 35));
        self.add(VerbEntry::regular("relate", Measurement, VerbGroup::Compare, Some(true), 55));
        self.add(VerbEntry::regular("equate", Measurement, VerbGroup::Compare, Some(true), 35));
        self.add(VerbEntry::regular("balance", Measurement, VerbGroup::Compare, Some(true), 50));
        self.add(VerbEntry::regular("verify", Measurement, VerbGroup::Compare, Some(true), 50));
        self.add(VerbEntry::regular("check", Measurement, VerbGroup::Compare, Some(true), 80));
        self.add(VerbEntry::regular("test", Measurement, VerbGroup::Compare, Some(true), 70));
        self.add(VerbEntry::regular("validate", Measurement, VerbGroup::Compare, Some(true), 45));
        self.add(VerbEntry::regular("confirm", Measurement, VerbGroup::Compare, Some(true), 60));

        // COUNT group
        self.add(VerbEntry::regular("count", Measurement, VerbGroup::Count, Some(true), 75)
            .with_synonyms(&["number", "tally", "enumerate", "total", "sum"]));
        self.add(VerbEntry::regular("number", Measurement, VerbGroup::Count, Some(true), 55));
        self.add(VerbEntry::regular("tally", Measurement, VerbGroup::Count, Some(true), 35));
        self.add(VerbEntry::regular("enumerate", Measurement, VerbGroup::Count, Some(true), 30));
        self.add(VerbEntry::regular("total", Measurement, VerbGroup::Count, Some(true), 50));
        self.add(VerbEntry::regular("sum", Measurement, VerbGroup::Count, Some(true), 45));
        self.add(VerbEntry::regular("add", Measurement, VerbGroup::Count, Some(true), 80));
        self.add(VerbEntry::regular("subtract", Measurement, VerbGroup::Count, Some(true), 50));
        self.add(VerbEntry::regular("multiply", Measurement, VerbGroup::Count, Some(true), 50));
        self.add(VerbEntry::regular("divide", Measurement, VerbGroup::Count, Some(true), 55));
        self.add(VerbEntry::regular("average", Measurement, VerbGroup::Count, Some(true), 45));
    }

    pub(super) fn load_modal_verbs(&mut self) {
        use FunctionalCategory::Modal;

        // WANT group
        self.add(VerbEntry::regular("want", Modal, VerbGroup::Want, Some(true), 95)
            .with_synonyms(&["wish", "desire", "crave", "long", "yearn"]));
        self.add(VerbEntry::regular("wish", Modal, VerbGroup::Want, Some(true), 75));
        self.add(VerbEntry::regular("desire", Modal, VerbGroup::Want, Some(true), 55));
        self.add(VerbEntry::regular("crave", Modal, VerbGroup::Want, Some(true), 45));
        self.add(VerbEntry::regular("long", Modal, VerbGroup::Want, None, 50));
        self.add(VerbEntry::regular("yearn", Modal, VerbGroup::Want, None, 35));
        self.add(VerbEntry::regular("prefer", Modal, VerbGroup::Want, Some(true), 60));
        self.add(VerbEntry::regular("fancy", Modal, VerbGroup::Want, Some(true), 40));
        self.add(VerbEntry::regular("covet", Modal, VerbGroup::Want, Some(true), 30));

        // NEED group
        self.add(VerbEntry::regular("need", Modal, VerbGroup::Need, Some(true), 90)
            .with_synonyms(&["require", "demand", "necessitate", "lack"]));
        self.add(VerbEntry::regular("require", Modal, VerbGroup::Need, Some(true), 70));
        self.add(VerbEntry::regular("demand", Modal, VerbGroup::Need, Some(true), 60));
        self.add(VerbEntry::regular("necessitate", Modal, VerbGroup::Need, Some(true), 30));
        self.add(VerbEntry::regular("lack", Modal, VerbGroup::Need, Some(true), 50));
        self.add(VerbEntry::regular("miss", Modal, VerbGroup::Need, Some(true), 70));

        // CAN group (ability/possibility)
        // Note: "can" is a true modal, handled specially in grammar
        self.add(VerbEntry::regular("manage", Modal, VerbGroup::Can, Some(true), 70));
        self.add(VerbEntry::regular("afford", Modal, VerbGroup::Can, Some(true), 55));

        // SHOULD group (obligation/expectation)
        // Note: "should" is a true modal, handled specially in grammar
        self.add(VerbEntry::regular("ought", Modal, VerbGroup::Should, None, 40));
        self.add(VerbEntry::regular("must", Modal, VerbGroup::Should, None, 75));

        // Additional semi-modals
        self.add(VerbEntry::regular("dare", Modal, VerbGroup::Can, Some(true), 50));
        self.add(VerbEntry::regular("tend", Modal, VerbGroup::Should, None, 55));
        self.add(VerbEntry::regular("seem", Modal, VerbGroup::Can, None, 70));
        self.add(VerbEntry::regular("appear", Modal, VerbGroup::Can, None, 65));
        self.add(VerbEntry::regular("happen", Modal, VerbGroup::Can, None, 70));
    }

    pub(super) fn load_weather_verbs(&mut self) {
        use FunctionalCategory::Weather;

        // RAIN group
        self.add(VerbEntry::regular("rain", Weather, VerbGroup::Rain, None, 65)
            .with_synonyms(&["pour", "drizzle", "shower", "sprinkle"]));
        self.add(VerbEntry::regular("pour", Weather, VerbGroup::Rain, None, 55));
        self.add(VerbEntry::regular("drizzle", Weather, VerbGroup::Rain, None, 35));
        self.add(VerbEntry::regular("shower", Weather, VerbGroup::Rain, None, 40));
        self.add(VerbEntry::regular("sprinkle", Weather, VerbGroup::Rain, None, 40));
        self.add(VerbEntry::regular("precipitate", Weather, VerbGroup::Rain, None, 25));
        self.add(VerbEntry::regular("storm", Weather, VerbGroup::Rain, None, 45));
        self.add(VerbEntry::regular("thunder", Weather, VerbGroup::Rain, None, 40));

        // SNOW group
        self.add(VerbEntry::regular("snow", Weather, VerbGroup::Snow, None, 55)
            .with_synonyms(&["blizzard", "flurry", "sleet", "hail"]));
        self.add(VerbEntry::regular("sleet", Weather, VerbGroup::Snow, None, 25));
        self.add(VerbEntry::regular("hail", Weather, VerbGroup::Snow, None, 35));
        self.add(VerbEntry::regular("frost", Weather, VerbGroup::Snow, None, 35));
        self.add(VerbEntry::regular("freeze", Weather, VerbGroup::Snow, None, 50));
        self.add(VerbEntry::regular("thaw", Weather, VerbGroup::Snow, None, 35));
        self.add(VerbEntry::regular("melt", Weather, VerbGroup::Snow, None, 50));

        // BLOW group
        self.add(VerbEntry::irregular("blow", "blew", "blown", Weather, VerbGroup::Blow, None, 60)
            .with_synonyms(&["gust", "breeze", "waft"]));
        self.add(VerbEntry::regular("gust", Weather, VerbGroup::Blow, None, 30));
        self.add(VerbEntry::regular("breeze", Weather, VerbGroup::Blow, None, 30));
        self.add(VerbEntry::regular("waft", Weather, VerbGroup::Blow, None, 25));
        self.add(VerbEntry::regular("swirl", Weather, VerbGroup::Blow, None, 35));
        self.add(VerbEntry::regular("howl", Weather, VerbGroup::Blow, None, 35));

        // General weather
        self.add(VerbEntry::regular("brighten", Weather, VerbGroup::Blow, None, 40));
        self.add(VerbEntry::regular("darken", Weather, VerbGroup::Blow, None, 40));
        self.add(VerbEntry::regular("clear", Weather, VerbGroup::Blow, None, 60));
        self.add(VerbEntry::regular("cloud", Weather, VerbGroup::Blow, None, 40));
        self.add(VerbEntry::regular("fog", Weather, VerbGroup::Blow, None, 35));
    }

    pub(super) fn load_emission_verbs(&mut self) {
        use FunctionalCategory::Emission;

        // SHINE group
        self.add(VerbEntry::irregular("shine", "shone", "shone", Emission, VerbGroup::Shine, None, 60)
            .with_synonyms(&["glow", "gleam", "glitter", "sparkle", "radiate", "beam"]));
        self.add(VerbEntry::regular("glow", Emission, VerbGroup::Shine, None, 50));
        self.add(VerbEntry::regular("gleam", Emission, VerbGroup::Shine, None, 40));
        self.add(VerbEntry::regular("glitter", Emission, VerbGroup::Shine, None, 35));
        self.add(VerbEntry::regular("sparkle", Emission, VerbGroup::Shine, None, 45));
        self.add(VerbEntry::regular("radiate", Emission, VerbGroup::Shine, None, 45));
        self.add(VerbEntry::regular("beam", Emission, VerbGroup::Shine, None, 45));
        self.add(VerbEntry::regular("flash", Emission, VerbGroup::Shine, None, 55));
        self.add(VerbEntry::regular("flicker", Emission, VerbGroup::Shine, None, 40));
        self.add(VerbEntry::regular("twinkle", Emission, VerbGroup::Shine, None, 35));
        self.add(VerbEntry::regular("shimmer", Emission, VerbGroup::Shine, None, 35));
        self.add(VerbEntry::regular("glisten", Emission, VerbGroup::Shine, None, 30));
        self.add(VerbEntry::regular("illuminate", Emission, VerbGroup::Shine, Some(true), 45));
        self.add(VerbEntry::irregular("light", "lit", "lit", Emission, VerbGroup::Shine, Some(true), 70));
        self.add(VerbEntry::regular("brighten", Emission, VerbGroup::Shine, None, 45));
        self.add(VerbEntry::regular("dim", Emission, VerbGroup::Shine, None, 40));
        self.add(VerbEntry::regular("fade", Emission, VerbGroup::Shine, None, 50));

        // SOUND group
        self.add(VerbEntry::regular("sound", Emission, VerbGroup::Sound, None, 70)
            .with_synonyms(&["ring", "buzz", "chime", "resonate", "echo"]));
        self.add(VerbEntry::irregular("ring", "rang", "rung", Emission, VerbGroup::Sound, None, 60));
        self.add(VerbEntry::regular("buzz", Emission, VerbGroup::Sound, None, 45));
        self.add(VerbEntry::regular("chime", Emission, VerbGroup::Sound, None, 35));
        self.add(VerbEntry::regular("resonate", Emission, VerbGroup::Sound, None, 35));
        self.add(VerbEntry::regular("echo", Emission, VerbGroup::Sound, None, 45));
        self.add(VerbEntry::regular("hum", Emission, VerbGroup::Sound, None, 45));
        self.add(VerbEntry::regular("vibrate", Emission, VerbGroup::Sound, None, 45));
        self.add(VerbEntry::regular("rumble", Emission, VerbGroup::Sound, None, 40));
        self.add(VerbEntry::regular("roar", Emission, VerbGroup::Sound, None, 45));
        self.add(VerbEntry::regular("thunder", Emission, VerbGroup::Sound, None, 40));
        self.add(VerbEntry::regular("boom", Emission, VerbGroup::Sound, None, 40));
        self.add(VerbEntry::regular("crash", Emission, VerbGroup::Sound, None, 55));
        self.add(VerbEntry::regular("clang", Emission, VerbGroup::Sound, None, 30));
        self.add(VerbEntry::regular("clatter", Emission, VerbGroup::Sound, None, 30));
        self.add(VerbEntry::regular("rattle", Emission, VerbGroup::Sound, None, 40));
        self.add(VerbEntry::regular("jingle", Emission, VerbGroup::Sound, None, 30));
        self.add(VerbEntry::regular("tinkle", Emission, VerbGroup::Sound, None, 25));
        self.add(VerbEntry::regular("click", Emission, VerbGroup::Sound, None, 50));
        self.add(VerbEntry::regular("clap", Emission, VerbGroup::Sound, None, 50));
        self.add(VerbEntry::regular("snap", Emission, VerbGroup::Sound, None, 50));
        self.add(VerbEntry::regular("pop", Emission, VerbGroup::Sound, None, 50));
        self.add(VerbEntry::regular("crackle", Emission, VerbGroup::Sound, None, 35));
        self.add(VerbEntry::regular("sizzle", Emission, VerbGroup::Sound, None, 35));
        self.add(VerbEntry::regular("hiss", Emission, VerbGroup::Sound, None, 40));
        self.add(VerbEntry::regular("whistle", Emission, VerbGroup::Sound, None, 45));
        self.add(VerbEntry::regular("squeak", Emission, VerbGroup::Sound, None, 40));
        self.add(VerbEntry::regular("squeal", Emission, VerbGroup::Sound, None, 35));
        self.add(VerbEntry::regular("screech", Emission, VerbGroup::Sound, None, 35));
        self.add(VerbEntry::regular("beep", Emission, VerbGroup::Sound, None, 45));
        self.add(VerbEntry::regular("honk", Emission, VerbGroup::Sound, None, 40));
    }
}
