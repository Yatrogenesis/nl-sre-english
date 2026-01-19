//! Complete English Verb Database - Part 1
//! Movement, Perception, Communication verbs

use super::{VerbDatabase, VerbEntry, FunctionalCategory, VerbGroup};

impl VerbDatabase {
    /// Load all built-in verbs
    pub fn load_builtin_verbs(&mut self) {
        self.load_movement_verbs();
        self.load_perception_verbs();
        self.load_communication_verbs();
        self.load_cognition_verbs();
        self.load_emotion_verbs();
        self.load_physical_verbs();
        self.load_state_verbs();
        self.load_transfer_verbs();
        self.load_creation_verbs();
        self.load_destruction_verbs();
        self.load_control_verbs();
        self.load_possession_verbs();
        self.load_social_verbs();
        self.load_consumption_verbs();
        self.load_body_verbs();
        self.load_aspectual_verbs();
        self.load_causation_verbs();
        self.load_attempt_verbs();
        self.load_connection_verbs();
        self.load_position_verbs();
        self.load_measurement_verbs();
        self.load_modal_verbs();
        self.load_weather_verbs();
        self.load_emission_verbs();
    }

    fn load_movement_verbs(&mut self) {
        use FunctionalCategory::Movement;

        // WALK group
        self.add(VerbEntry::regular("walk", Movement, VerbGroup::Walk, None, 95)
            .with_synonyms(&["stroll", "amble", "saunter", "pace", "march", "stride", "trudge", "wander"]));
        self.add(VerbEntry::regular("stroll", Movement, VerbGroup::Walk, None, 60));
        self.add(VerbEntry::regular("amble", Movement, VerbGroup::Walk, None, 30));
        self.add(VerbEntry::regular("saunter", Movement, VerbGroup::Walk, None, 25));
        self.add(VerbEntry::regular("pace", Movement, VerbGroup::Walk, None, 55));
        self.add(VerbEntry::regular("march", Movement, VerbGroup::Walk, None, 65));
        self.add(VerbEntry::irregular("stride", "strode", "stridden", Movement, VerbGroup::Walk, None, 45));
        self.add(VerbEntry::regular("trudge", Movement, VerbGroup::Walk, None, 35));
        self.add(VerbEntry::regular("wander", Movement, VerbGroup::Walk, None, 60));
        self.add(VerbEntry::regular("roam", Movement, VerbGroup::Walk, None, 45));
        self.add(VerbEntry::regular("hike", Movement, VerbGroup::Walk, None, 55));
        self.add(VerbEntry::regular("trek", Movement, VerbGroup::Walk, None, 40));
        self.add(VerbEntry::regular("tiptoe", Movement, VerbGroup::Walk, None, 35));
        self.add(VerbEntry::regular("shuffle", Movement, VerbGroup::Walk, None, 40));
        self.add(VerbEntry::regular("limp", Movement, VerbGroup::Walk, None, 45));
        self.add(VerbEntry::regular("stagger", Movement, VerbGroup::Walk, None, 40));
        self.add(VerbEntry::regular("stumble", Movement, VerbGroup::Walk, None, 50));

        // RUN group
        self.add(VerbEntry::irregular("run", "ran", "run", Movement, VerbGroup::Run, None, 98)
            .with_synonyms(&["sprint", "dash", "race", "jog", "rush", "hurry", "bolt", "flee"]));
        self.add(VerbEntry::regular("sprint", Movement, VerbGroup::Run, None, 60));
        self.add(VerbEntry::regular("dash", Movement, VerbGroup::Run, None, 55));
        self.add(VerbEntry::regular("race", Movement, VerbGroup::Run, None, 65));
        self.add(VerbEntry::regular("jog", Movement, VerbGroup::Run, None, 60));
        self.add(VerbEntry::regular("rush", Movement, VerbGroup::Run, None, 70));
        self.add(VerbEntry::regular("hurry", Movement, VerbGroup::Run, None, 70));
        self.add(VerbEntry::regular("bolt", Movement, VerbGroup::Run, None, 45));
        self.add(VerbEntry::irregular("flee", "fled", "fled", Movement, VerbGroup::Run, None, 55));
        self.add(VerbEntry::regular("gallop", Movement, VerbGroup::Run, None, 40));
        self.add(VerbEntry::regular("scurry", Movement, VerbGroup::Run, None, 35));
        self.add(VerbEntry::regular("scamper", Movement, VerbGroup::Run, None, 30));
        self.add(VerbEntry::regular("chase", Movement, VerbGroup::Run, Some(true), 65));
        self.add(VerbEntry::regular("pursue", Movement, VerbGroup::Run, Some(true), 55));

        // JUMP group
        self.add(VerbEntry::regular("jump", Movement, VerbGroup::Jump, None, 80)
            .with_synonyms(&["leap", "hop", "skip", "bounce", "spring", "vault", "bound"]));
        self.add(VerbEntry::irregular("leap", "leapt", "leapt", Movement, VerbGroup::Jump, None, 60));
        self.add(VerbEntry::regular("hop", Movement, VerbGroup::Jump, None, 55));
        self.add(VerbEntry::regular("skip", Movement, VerbGroup::Jump, None, 55));
        self.add(VerbEntry::regular("bounce", Movement, VerbGroup::Jump, None, 55));
        self.add(VerbEntry::irregular("spring", "sprang", "sprung", Movement, VerbGroup::Jump, None, 50));
        self.add(VerbEntry::regular("vault", Movement, VerbGroup::Jump, None, 35));
        self.add(VerbEntry::regular("bound", Movement, VerbGroup::Jump, None, 45));
        self.add(VerbEntry::regular("pounce", Movement, VerbGroup::Jump, None, 40));
        self.add(VerbEntry::regular("lunge", Movement, VerbGroup::Jump, None, 40));

        // FLY group
        self.add(VerbEntry::irregular("fly", "flew", "flown", Movement, VerbGroup::Fly, None, 80)
            .with_synonyms(&["soar", "glide", "hover", "flutter", "swoop", "float"]));
        self.add(VerbEntry::regular("soar", Movement, VerbGroup::Fly, None, 50));
        self.add(VerbEntry::regular("glide", Movement, VerbGroup::Fly, None, 45));
        self.add(VerbEntry::regular("hover", Movement, VerbGroup::Fly, None, 50));
        self.add(VerbEntry::regular("flutter", Movement, VerbGroup::Fly, None, 40));
        self.add(VerbEntry::regular("swoop", Movement, VerbGroup::Fly, None, 35));
        self.add(VerbEntry::regular("float", Movement, VerbGroup::Fly, None, 55));
        self.add(VerbEntry::regular("ascend", Movement, VerbGroup::Fly, None, 45));
        self.add(VerbEntry::regular("descend", Movement, VerbGroup::Fly, None, 50));

        // SWIM group
        self.add(VerbEntry::irregular("swim", "swam", "swum", Movement, VerbGroup::Swim, None, 70)
            .with_synonyms(&["dive", "float", "wade", "paddle", "plunge"]));
        self.add(VerbEntry::irregular("dive", "dove", "dived", Movement, VerbGroup::Swim, None, 55));
        self.add(VerbEntry::regular("wade", Movement, VerbGroup::Swim, None, 40));
        self.add(VerbEntry::regular("paddle", Movement, VerbGroup::Swim, None, 40));
        self.add(VerbEntry::regular("plunge", Movement, VerbGroup::Swim, None, 45));
        self.add(VerbEntry::irregular("sink", "sank", "sunk", Movement, VerbGroup::Swim, None, 55));
        self.add(VerbEntry::regular("drown", Movement, VerbGroup::Swim, None, 45));
        self.add(VerbEntry::regular("submerge", Movement, VerbGroup::Swim, Some(true), 35));

        // CLIMB group
        self.add(VerbEntry::regular("climb", Movement, VerbGroup::Climb, None, 65)
            .with_synonyms(&["ascend", "scale", "mount", "clamber"]));
        self.add(VerbEntry::regular("scale", Movement, VerbGroup::Climb, Some(true), 45));
        self.add(VerbEntry::regular("mount", Movement, VerbGroup::Climb, Some(true), 50));
        self.add(VerbEntry::regular("clamber", Movement, VerbGroup::Climb, None, 30));
        self.add(VerbEntry::regular("scramble", Movement, VerbGroup::Climb, None, 45));

        // FALL group
        self.add(VerbEntry::irregular("fall", "fell", "fallen", Movement, VerbGroup::Fall, None, 80)
            .with_synonyms(&["drop", "tumble", "collapse", "plummet", "descend", "trip"]));
        self.add(VerbEntry::regular("drop", Movement, VerbGroup::Fall, None, 75));
        self.add(VerbEntry::regular("tumble", Movement, VerbGroup::Fall, None, 45));
        self.add(VerbEntry::regular("collapse", Movement, VerbGroup::Fall, None, 55));
        self.add(VerbEntry::regular("plummet", Movement, VerbGroup::Fall, None, 35));
        self.add(VerbEntry::regular("trip", Movement, VerbGroup::Fall, None, 55));
        self.add(VerbEntry::regular("slip", Movement, VerbGroup::Fall, None, 60));
        self.add(VerbEntry::regular("topple", Movement, VerbGroup::Fall, None, 35));

        // TURN group
        self.add(VerbEntry::regular("turn", Movement, VerbGroup::Turn, None, 90)
            .with_synonyms(&["rotate", "spin", "twist", "pivot", "revolve", "swivel"]));
        self.add(VerbEntry::regular("rotate", Movement, VerbGroup::Turn, None, 55));
        self.add(VerbEntry::irregular("spin", "spun", "spun", Movement, VerbGroup::Turn, None, 60));
        self.add(VerbEntry::regular("twist", Movement, VerbGroup::Turn, None, 55));
        self.add(VerbEntry::regular("pivot", Movement, VerbGroup::Turn, None, 40));
        self.add(VerbEntry::regular("revolve", Movement, VerbGroup::Turn, None, 40));
        self.add(VerbEntry::regular("swivel", Movement, VerbGroup::Turn, None, 35));
        self.add(VerbEntry::regular("roll", Movement, VerbGroup::Turn, None, 65));
        self.add(VerbEntry::regular("whirl", Movement, VerbGroup::Turn, None, 35));

        // ENTER group
        self.add(VerbEntry::irregular("come", "came", "come", Movement, VerbGroup::Enter, None, 98)
            .with_synonyms(&["arrive", "approach", "enter", "reach", "appear"]));
        self.add(VerbEntry::regular("arrive", Movement, VerbGroup::Enter, None, 75));
        self.add(VerbEntry::regular("approach", Movement, VerbGroup::Enter, Some(true), 60));
        self.add(VerbEntry::regular("enter", Movement, VerbGroup::Enter, Some(true), 75));
        self.add(VerbEntry::regular("reach", Movement, VerbGroup::Enter, Some(true), 75));
        self.add(VerbEntry::regular("appear", Movement, VerbGroup::Enter, None, 70));
        self.add(VerbEntry::regular("emerge", Movement, VerbGroup::Enter, None, 50));
        self.add(VerbEntry::regular("penetrate", Movement, VerbGroup::Enter, Some(true), 40));

        // EXIT group
        self.add(VerbEntry::irregular("go", "went", "gone", Movement, VerbGroup::Exit, None, 99)
            .with_synonyms(&["leave", "depart", "exit", "withdraw", "retreat"]));
        self.add(VerbEntry::irregular("leave", "left", "left", Movement, VerbGroup::Exit, Some(true), 90));
        self.add(VerbEntry::regular("depart", Movement, VerbGroup::Exit, None, 55));
        self.add(VerbEntry::regular("exit", Movement, VerbGroup::Exit, Some(true), 60));
        self.add(VerbEntry::irregular("withdraw", "withdrew", "withdrawn", Movement, VerbGroup::Exit, None, 50));
        self.add(VerbEntry::regular("retreat", Movement, VerbGroup::Exit, None, 45));
        self.add(VerbEntry::regular("escape", Movement, VerbGroup::Exit, None, 60));
        self.add(VerbEntry::regular("vanish", Movement, VerbGroup::Exit, None, 45));
        self.add(VerbEntry::regular("disappear", Movement, VerbGroup::Exit, None, 60));

        // Additional general movement
        self.add(VerbEntry::regular("move", Movement, VerbGroup::Walk, None, 95));
        self.add(VerbEntry::regular("travel", Movement, VerbGroup::Walk, None, 75));
        self.add(VerbEntry::irregular("ride", "rode", "ridden", Movement, VerbGroup::Walk, Some(true), 70));
        self.add(VerbEntry::irregular("drive", "drove", "driven", Movement, VerbGroup::Walk, Some(true), 85));
        self.add(VerbEntry::regular("sail", Movement, VerbGroup::Swim, None, 50));
        self.add(VerbEntry::regular("crawl", Movement, VerbGroup::Walk, None, 50));
        self.add(VerbEntry::regular("creep", Movement, VerbGroup::Walk, None, 45));
        self.add(VerbEntry::regular("slide", Movement, VerbGroup::Fall, None, 55));
        self.add(VerbEntry::regular("skate", Movement, VerbGroup::Walk, None, 40));
        self.add(VerbEntry::regular("ski", Movement, VerbGroup::Walk, None, 35));
    }

    fn load_perception_verbs(&mut self) {
        use FunctionalCategory::Perception;

        // SEE group
        self.add(VerbEntry::irregular("see", "saw", "seen", Perception, VerbGroup::See, Some(true), 99)
            .with_synonyms(&["watch", "observe", "notice", "view", "witness", "spot", "glimpse"]));
        self.add(VerbEntry::regular("watch", Perception, VerbGroup::See, Some(true), 90));
        self.add(VerbEntry::regular("observe", Perception, VerbGroup::See, Some(true), 65));
        self.add(VerbEntry::regular("notice", Perception, VerbGroup::See, Some(true), 75));
        self.add(VerbEntry::regular("view", Perception, VerbGroup::See, Some(true), 60));
        self.add(VerbEntry::regular("witness", Perception, VerbGroup::See, Some(true), 55));
        self.add(VerbEntry::regular("spot", Perception, VerbGroup::See, Some(true), 60));
        self.add(VerbEntry::regular("glimpse", Perception, VerbGroup::See, Some(true), 45));
        self.add(VerbEntry::regular("stare", Perception, VerbGroup::See, None, 55));
        self.add(VerbEntry::regular("gaze", Perception, VerbGroup::See, None, 45));
        self.add(VerbEntry::regular("glance", Perception, VerbGroup::See, None, 55));
        self.add(VerbEntry::regular("peek", Perception, VerbGroup::See, None, 45));
        self.add(VerbEntry::regular("peer", Perception, VerbGroup::See, None, 40));
        self.add(VerbEntry::regular("look", Perception, VerbGroup::See, None, 95));
        self.add(VerbEntry::regular("examine", Perception, VerbGroup::See, Some(true), 60));
        self.add(VerbEntry::regular("inspect", Perception, VerbGroup::See, Some(true), 50));
        self.add(VerbEntry::regular("scan", Perception, VerbGroup::See, Some(true), 55));
        self.add(VerbEntry::regular("survey", Perception, VerbGroup::See, Some(true), 45));
        self.add(VerbEntry::regular("perceive", Perception, VerbGroup::See, Some(true), 50));
        self.add(VerbEntry::regular("discern", Perception, VerbGroup::See, Some(true), 35));
        self.add(VerbEntry::regular("detect", Perception, VerbGroup::See, Some(true), 55));
        self.add(VerbEntry::regular("recognize", Perception, VerbGroup::See, Some(true), 65));
        self.add(VerbEntry::regular("identify", Perception, VerbGroup::See, Some(true), 60));

        // HEAR group
        self.add(VerbEntry::irregular("hear", "heard", "heard", Perception, VerbGroup::Hear, Some(true), 90)
            .with_synonyms(&["listen", "overhear", "eavesdrop"]));
        self.add(VerbEntry::regular("listen", Perception, VerbGroup::Hear, None, 80));
        self.add(VerbEntry::regular("overhear", Perception, VerbGroup::Hear, Some(true), 45));
        self.add(VerbEntry::regular("eavesdrop", Perception, VerbGroup::Hear, None, 30));

        // FEEL group
        self.add(VerbEntry::irregular("feel", "felt", "felt", Perception, VerbGroup::Feel, Some(true), 90)
            .with_synonyms(&["touch", "sense", "experience", "perceive"]));
        self.add(VerbEntry::regular("touch", Perception, VerbGroup::Feel, Some(true), 75));
        self.add(VerbEntry::regular("sense", Perception, VerbGroup::Feel, Some(true), 60));
        self.add(VerbEntry::regular("experience", Perception, VerbGroup::Feel, Some(true), 65));

        // SMELL group
        self.add(VerbEntry::regular("smell", Perception, VerbGroup::Smell, Some(true), 65)
            .with_synonyms(&["sniff", "scent", "inhale"]));
        self.add(VerbEntry::regular("sniff", Perception, VerbGroup::Smell, Some(true), 45));
        self.add(VerbEntry::regular("scent", Perception, VerbGroup::Smell, Some(true), 35));

        // TASTE group
        self.add(VerbEntry::regular("taste", Perception, VerbGroup::Taste, Some(true), 60)
            .with_synonyms(&["sample", "savor", "try"]));
        self.add(VerbEntry::regular("sample", Perception, VerbGroup::Taste, Some(true), 45));
        self.add(VerbEntry::regular("savor", Perception, VerbGroup::Taste, Some(true), 35));
    }

    fn load_communication_verbs(&mut self) {
        use FunctionalCategory::Communication;

        // SPEAK group
        self.add(VerbEntry::irregular("say", "said", "said", Communication, VerbGroup::Speak, Some(true), 99)
            .with_synonyms(&["tell", "speak", "talk", "utter", "state", "express", "declare"]));
        self.add(VerbEntry::irregular("tell", "told", "told", Communication, VerbGroup::Speak, Some(true), 95));
        self.add(VerbEntry::irregular("speak", "spoke", "spoken", Communication, VerbGroup::Speak, None, 85));
        self.add(VerbEntry::regular("talk", Communication, VerbGroup::Speak, None, 90));
        self.add(VerbEntry::regular("utter", Communication, VerbGroup::Speak, Some(true), 40));
        self.add(VerbEntry::regular("state", Communication, VerbGroup::Speak, Some(true), 60));
        self.add(VerbEntry::regular("express", Communication, VerbGroup::Speak, Some(true), 65));
        self.add(VerbEntry::regular("declare", Communication, VerbGroup::Speak, Some(true), 50));
        self.add(VerbEntry::regular("announce", Communication, VerbGroup::Speak, Some(true), 60));
        self.add(VerbEntry::regular("pronounce", Communication, VerbGroup::Speak, Some(true), 45));
        self.add(VerbEntry::regular("mention", Communication, VerbGroup::Speak, Some(true), 65));
        self.add(VerbEntry::regular("remark", Communication, VerbGroup::Speak, Some(true), 50));
        self.add(VerbEntry::regular("comment", Communication, VerbGroup::Speak, None, 60));
        self.add(VerbEntry::regular("chat", Communication, VerbGroup::Speak, None, 60));
        self.add(VerbEntry::regular("converse", Communication, VerbGroup::Speak, None, 40));
        self.add(VerbEntry::regular("communicate", Communication, VerbGroup::Speak, None, 60));
        self.add(VerbEntry::regular("narrate", Communication, VerbGroup::Speak, Some(true), 35));
        self.add(VerbEntry::regular("report", Communication, VerbGroup::Speak, Some(true), 65));
        self.add(VerbEntry::regular("whisper", Communication, VerbGroup::Speak, Some(true), 50));
        self.add(VerbEntry::regular("shout", Communication, VerbGroup::Speak, Some(true), 55));
        self.add(VerbEntry::regular("yell", Communication, VerbGroup::Speak, Some(true), 55));
        self.add(VerbEntry::regular("scream", Communication, VerbGroup::Speak, Some(true), 55));
        self.add(VerbEntry::regular("murmur", Communication, VerbGroup::Speak, Some(true), 40));
        self.add(VerbEntry::regular("mumble", Communication, VerbGroup::Speak, Some(true), 40));
        self.add(VerbEntry::regular("mutter", Communication, VerbGroup::Speak, Some(true), 40));

        // ASK group
        self.add(VerbEntry::regular("ask", Communication, VerbGroup::Ask, Some(true), 90)
            .with_synonyms(&["question", "inquire", "query", "request", "demand"]));
        self.add(VerbEntry::regular("question", Communication, VerbGroup::Ask, Some(true), 60));
        self.add(VerbEntry::regular("inquire", Communication, VerbGroup::Ask, None, 45));
        self.add(VerbEntry::regular("query", Communication, VerbGroup::Ask, Some(true), 40));
        self.add(VerbEntry::regular("request", Communication, VerbGroup::Ask, Some(true), 65));
        self.add(VerbEntry::regular("demand", Communication, VerbGroup::Ask, Some(true), 60));
        self.add(VerbEntry::regular("beg", Communication, VerbGroup::Ask, Some(true), 50));
        self.add(VerbEntry::regular("plead", Communication, VerbGroup::Ask, None, 45));
        self.add(VerbEntry::regular("implore", Communication, VerbGroup::Ask, Some(true), 35));
        self.add(VerbEntry::regular("interview", Communication, VerbGroup::Ask, Some(true), 50));

        // ANSWER group
        self.add(VerbEntry::regular("answer", Communication, VerbGroup::Answer, Some(true), 80)
            .with_synonyms(&["reply", "respond", "retort"]));
        self.add(VerbEntry::regular("reply", Communication, VerbGroup::Answer, None, 65));
        self.add(VerbEntry::regular("respond", Communication, VerbGroup::Answer, None, 65));
        self.add(VerbEntry::regular("retort", Communication, VerbGroup::Answer, Some(true), 35));
        self.add(VerbEntry::regular("react", Communication, VerbGroup::Answer, None, 55));

        // EXPLAIN group
        self.add(VerbEntry::regular("explain", Communication, VerbGroup::Explain, Some(true), 75)
            .with_synonyms(&["describe", "clarify", "illustrate", "elaborate", "define"]));
        self.add(VerbEntry::regular("describe", Communication, VerbGroup::Explain, Some(true), 70));
        self.add(VerbEntry::regular("clarify", Communication, VerbGroup::Explain, Some(true), 50));
        self.add(VerbEntry::regular("illustrate", Communication, VerbGroup::Explain, Some(true), 50));
        self.add(VerbEntry::regular("elaborate", Communication, VerbGroup::Explain, None, 45));
        self.add(VerbEntry::regular("define", Communication, VerbGroup::Explain, Some(true), 55));
        self.add(VerbEntry::regular("interpret", Communication, VerbGroup::Explain, Some(true), 50));
        self.add(VerbEntry::regular("translate", Communication, VerbGroup::Explain, Some(true), 55));
        self.add(VerbEntry::regular("summarize", Communication, VerbGroup::Explain, Some(true), 50));
        self.add(VerbEntry::regular("outline", Communication, VerbGroup::Explain, Some(true), 45));

        // ARGUE group
        self.add(VerbEntry::regular("argue", Communication, VerbGroup::Argue, None, 65)
            .with_synonyms(&["debate", "discuss", "dispute", "disagree", "quarrel"]));
        self.add(VerbEntry::regular("debate", Communication, VerbGroup::Argue, Some(true), 50));
        self.add(VerbEntry::regular("discuss", Communication, VerbGroup::Argue, Some(true), 70));
        self.add(VerbEntry::regular("dispute", Communication, VerbGroup::Argue, Some(true), 45));
        self.add(VerbEntry::regular("disagree", Communication, VerbGroup::Argue, None, 55));
        self.add(VerbEntry::regular("quarrel", Communication, VerbGroup::Argue, None, 40));
        self.add(VerbEntry::regular("bicker", Communication, VerbGroup::Argue, None, 30));
        self.add(VerbEntry::regular("contend", Communication, VerbGroup::Argue, None, 40));
        self.add(VerbEntry::regular("assert", Communication, VerbGroup::Argue, Some(true), 50));
        self.add(VerbEntry::regular("claim", Communication, VerbGroup::Argue, Some(true), 65));
        self.add(VerbEntry::regular("maintain", Communication, VerbGroup::Argue, Some(true), 55));
        self.add(VerbEntry::regular("insist", Communication, VerbGroup::Argue, None, 55));

        // PROMISE group
        self.add(VerbEntry::regular("promise", Communication, VerbGroup::Promise, Some(true), 70)
            .with_synonyms(&["vow", "swear", "pledge", "guarantee"]));
        self.add(VerbEntry::regular("vow", Communication, VerbGroup::Promise, Some(true), 45));
        self.add(VerbEntry::irregular("swear", "swore", "sworn", Communication, VerbGroup::Promise, Some(true), 55));
        self.add(VerbEntry::regular("pledge", Communication, VerbGroup::Promise, Some(true), 45));
        self.add(VerbEntry::regular("guarantee", Communication, VerbGroup::Promise, Some(true), 55));
        self.add(VerbEntry::regular("assure", Communication, VerbGroup::Promise, Some(true), 55));

        // WARN group
        self.add(VerbEntry::regular("warn", Communication, VerbGroup::Warn, Some(true), 65)
            .with_synonyms(&["caution", "alert", "advise", "notify"]));
        self.add(VerbEntry::regular("caution", Communication, VerbGroup::Warn, Some(true), 45));
        self.add(VerbEntry::regular("alert", Communication, VerbGroup::Warn, Some(true), 55));
        self.add(VerbEntry::regular("advise", Communication, VerbGroup::Warn, Some(true), 60));
        self.add(VerbEntry::regular("notify", Communication, VerbGroup::Warn, Some(true), 50));
        self.add(VerbEntry::regular("inform", Communication, VerbGroup::Warn, Some(true), 60));
        self.add(VerbEntry::regular("remind", Communication, VerbGroup::Warn, Some(true), 65));

        // COMMAND group
        self.add(VerbEntry::regular("command", Communication, VerbGroup::Command, Some(true), 55)
            .with_synonyms(&["order", "instruct", "direct", "dictate"]));
        self.add(VerbEntry::regular("order", Communication, VerbGroup::Command, Some(true), 70));
        self.add(VerbEntry::regular("instruct", Communication, VerbGroup::Command, Some(true), 55));
        self.add(VerbEntry::regular("direct", Communication, VerbGroup::Command, Some(true), 60));
        self.add(VerbEntry::regular("dictate", Communication, VerbGroup::Command, Some(true), 40));
        self.add(VerbEntry::irregular("forbid", "forbade", "forbidden", Communication, VerbGroup::Command, Some(true), 45));
        self.add(VerbEntry::regular("prohibit", Communication, VerbGroup::Command, Some(true), 45));
        self.add(VerbEntry::regular("permit", Communication, VerbGroup::Command, Some(true), 50));

        // SUGGEST group
        self.add(VerbEntry::regular("suggest", Communication, VerbGroup::Suggest, Some(true), 70)
            .with_synonyms(&["propose", "recommend", "hint", "imply"]));
        self.add(VerbEntry::regular("propose", Communication, VerbGroup::Suggest, Some(true), 55));
        self.add(VerbEntry::regular("recommend", Communication, VerbGroup::Suggest, Some(true), 60));
        self.add(VerbEntry::regular("hint", Communication, VerbGroup::Suggest, None, 50));
        self.add(VerbEntry::regular("imply", Communication, VerbGroup::Suggest, Some(true), 50));
        self.add(VerbEntry::regular("indicate", Communication, VerbGroup::Suggest, Some(true), 55));
        self.add(VerbEntry::regular("insinuate", Communication, VerbGroup::Suggest, Some(true), 30));

        // Additional communication verbs
        self.add(VerbEntry::regular("agree", Communication, VerbGroup::Answer, None, 75));
        self.add(VerbEntry::regular("deny", Communication, VerbGroup::Argue, Some(true), 55));
        self.add(VerbEntry::regular("admit", Communication, VerbGroup::Speak, Some(true), 60));
        self.add(VerbEntry::regular("confess", Communication, VerbGroup::Speak, Some(true), 45));
        self.add(VerbEntry::regular("reveal", Communication, VerbGroup::Speak, Some(true), 55));
        self.add(VerbEntry::regular("disclose", Communication, VerbGroup::Speak, Some(true), 40));
        self.add(VerbEntry::regular("conceal", Communication, VerbGroup::Speak, Some(true), 40));
        self.add(VerbEntry::irregular("lie", "lied", "lied", Communication, VerbGroup::Speak, None, 55));
        self.add(VerbEntry::regular("deceive", Communication, VerbGroup::Speak, Some(true), 40));
        self.add(VerbEntry::regular("convince", Communication, VerbGroup::Argue, Some(true), 60));
        self.add(VerbEntry::regular("persuade", Communication, VerbGroup::Argue, Some(true), 55));
        self.add(VerbEntry::regular("encourage", Communication, VerbGroup::Suggest, Some(true), 65));
        self.add(VerbEntry::regular("discourage", Communication, VerbGroup::Warn, Some(true), 50));
        self.add(VerbEntry::regular("criticize", Communication, VerbGroup::Argue, Some(true), 55));
        self.add(VerbEntry::regular("praise", Communication, VerbGroup::Speak, Some(true), 55));
        self.add(VerbEntry::regular("compliment", Communication, VerbGroup::Speak, Some(true), 50));
        self.add(VerbEntry::regular("thank", Communication, VerbGroup::Speak, Some(true), 75));
        self.add(VerbEntry::regular("apologize", Communication, VerbGroup::Speak, None, 55));
        self.add(VerbEntry::regular("greet", Communication, VerbGroup::Speak, Some(true), 55));
        self.add(VerbEntry::regular("introduce", Communication, VerbGroup::Speak, Some(true), 60));
        self.add(VerbEntry::regular("invite", Communication, VerbGroup::Ask, Some(true), 65));
        self.add(VerbEntry::regular("accept", Communication, VerbGroup::Answer, Some(true), 70));
        self.add(VerbEntry::regular("refuse", Communication, VerbGroup::Answer, Some(true), 55));
        self.add(VerbEntry::regular("reject", Communication, VerbGroup::Answer, Some(true), 55));
        self.add(VerbEntry::regular("confirm", Communication, VerbGroup::Answer, Some(true), 60));
        self.add(VerbEntry::regular("cancel", Communication, VerbGroup::Command, Some(true), 60));
    }
}
