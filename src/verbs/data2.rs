//! Complete English Verb Database - Part 2
//! Cognition, Emotion, Physical, State verbs

use super::{VerbDatabase, VerbEntry, FunctionalCategory, VerbGroup};

impl VerbDatabase {
    pub(super) fn load_cognition_verbs(&mut self) {
        use FunctionalCategory::Cognition;

        // THINK group
        self.add(VerbEntry::irregular("think", "thought", "thought", Cognition, VerbGroup::Think, None, 98)
            .with_synonyms(&["ponder", "consider", "contemplate", "reflect", "meditate", "reason"]));
        self.add(VerbEntry::regular("ponder", Cognition, VerbGroup::Think, Some(true), 40));
        self.add(VerbEntry::regular("consider", Cognition, VerbGroup::Think, Some(true), 75));
        self.add(VerbEntry::regular("contemplate", Cognition, VerbGroup::Think, Some(true), 40));
        self.add(VerbEntry::regular("reflect", Cognition, VerbGroup::Think, None, 55));
        self.add(VerbEntry::regular("meditate", Cognition, VerbGroup::Think, None, 40));
        self.add(VerbEntry::regular("reason", Cognition, VerbGroup::Think, None, 45));
        self.add(VerbEntry::regular("wonder", Cognition, VerbGroup::Think, None, 65));
        self.add(VerbEntry::regular("speculate", Cognition, VerbGroup::Think, None, 40));
        self.add(VerbEntry::regular("muse", Cognition, VerbGroup::Think, None, 25));
        self.add(VerbEntry::regular("deliberate", Cognition, VerbGroup::Think, None, 35));
        self.add(VerbEntry::regular("concentrate", Cognition, VerbGroup::Think, None, 55));
        self.add(VerbEntry::regular("focus", Cognition, VerbGroup::Think, None, 70));

        // KNOW group
        self.add(VerbEntry::irregular("know", "knew", "known", Cognition, VerbGroup::Know, Some(true), 99)
            .with_synonyms(&["understand", "comprehend", "grasp", "realize", "recognize", "perceive"]));
        self.add(VerbEntry::irregular("understand", "understood", "understood", Cognition, VerbGroup::Know, Some(true), 90));
        self.add(VerbEntry::regular("comprehend", Cognition, VerbGroup::Know, Some(true), 45));
        self.add(VerbEntry::regular("grasp", Cognition, VerbGroup::Know, Some(true), 50));
        self.add(VerbEntry::regular("realize", Cognition, VerbGroup::Know, Some(true), 70));
        self.add(VerbEntry::regular("fathom", Cognition, VerbGroup::Know, Some(true), 25));
        self.add(VerbEntry::regular("appreciate", Cognition, VerbGroup::Know, Some(true), 60));
        self.add(VerbEntry::regular("acknowledge", Cognition, VerbGroup::Know, Some(true), 55));

        // BELIEVE group
        self.add(VerbEntry::regular("believe", Cognition, VerbGroup::Believe, Some(true), 85)
            .with_synonyms(&["suppose", "assume", "presume", "expect", "suspect", "guess"]));
        self.add(VerbEntry::regular("suppose", Cognition, VerbGroup::Believe, Some(true), 60));
        self.add(VerbEntry::regular("assume", Cognition, VerbGroup::Believe, Some(true), 65));
        self.add(VerbEntry::regular("presume", Cognition, VerbGroup::Believe, Some(true), 40));
        self.add(VerbEntry::regular("expect", Cognition, VerbGroup::Believe, Some(true), 80));
        self.add(VerbEntry::regular("suspect", Cognition, VerbGroup::Believe, Some(true), 55));
        self.add(VerbEntry::regular("guess", Cognition, VerbGroup::Believe, Some(true), 65));
        self.add(VerbEntry::regular("estimate", Cognition, VerbGroup::Believe, Some(true), 55));
        self.add(VerbEntry::regular("predict", Cognition, VerbGroup::Believe, Some(true), 55));
        self.add(VerbEntry::regular("anticipate", Cognition, VerbGroup::Believe, Some(true), 50));
        self.add(VerbEntry::regular("doubt", Cognition, VerbGroup::Believe, Some(true), 60));
        self.add(VerbEntry::regular("trust", Cognition, VerbGroup::Believe, Some(true), 70));

        // REMEMBER group
        self.add(VerbEntry::regular("remember", Cognition, VerbGroup::Remember, Some(true), 85)
            .with_synonyms(&["recall", "recollect", "reminisce", "retain"]));
        self.add(VerbEntry::regular("recall", Cognition, VerbGroup::Remember, Some(true), 60));
        self.add(VerbEntry::regular("recollect", Cognition, VerbGroup::Remember, Some(true), 30));
        self.add(VerbEntry::regular("reminisce", Cognition, VerbGroup::Remember, None, 30));
        self.add(VerbEntry::regular("retain", Cognition, VerbGroup::Remember, Some(true), 45));
        self.add(VerbEntry::regular("memorize", Cognition, VerbGroup::Remember, Some(true), 50));

        // FORGET group
        self.add(VerbEntry::irregular("forget", "forgot", "forgotten", Cognition, VerbGroup::Forget, Some(true), 75)
            .with_synonyms(&["overlook", "neglect", "disregard", "ignore"]));
        self.add(VerbEntry::regular("overlook", Cognition, VerbGroup::Forget, Some(true), 50));
        self.add(VerbEntry::regular("neglect", Cognition, VerbGroup::Forget, Some(true), 45));
        self.add(VerbEntry::regular("disregard", Cognition, VerbGroup::Forget, Some(true), 40));
        self.add(VerbEntry::regular("ignore", Cognition, VerbGroup::Forget, Some(true), 65));

        // LEARN group
        self.add(VerbEntry::irregular("learn", "learned", "learned", Cognition, VerbGroup::Learn, Some(true), 85)
            .with_synonyms(&["study", "discover", "find", "ascertain", "determine"]));
        self.add(VerbEntry::regular("study", Cognition, VerbGroup::Learn, Some(true), 75));
        self.add(VerbEntry::regular("discover", Cognition, VerbGroup::Learn, Some(true), 65));
        self.add(VerbEntry::irregular("find", "found", "found", Cognition, VerbGroup::Learn, Some(true), 90));
        self.add(VerbEntry::regular("ascertain", Cognition, VerbGroup::Learn, Some(true), 30));
        self.add(VerbEntry::regular("determine", Cognition, VerbGroup::Learn, Some(true), 60));
        self.add(VerbEntry::regular("research", Cognition, VerbGroup::Learn, Some(true), 55));
        self.add(VerbEntry::regular("explore", Cognition, VerbGroup::Learn, Some(true), 60));
        self.add(VerbEntry::regular("investigate", Cognition, VerbGroup::Learn, Some(true), 55));
        self.add(VerbEntry::regular("educate", Cognition, VerbGroup::Learn, Some(true), 50));
        self.add(VerbEntry::regular("teach", Cognition, VerbGroup::Learn, Some(true), 80));
        self.add(VerbEntry::regular("train", Cognition, VerbGroup::Learn, Some(true), 65));

        // DECIDE group
        self.add(VerbEntry::regular("decide", Cognition, VerbGroup::Decide, Some(true), 80)
            .with_synonyms(&["choose", "determine", "resolve", "conclude", "select", "opt"]));
        self.add(VerbEntry::irregular("choose", "chose", "chosen", Cognition, VerbGroup::Decide, Some(true), 80));
        self.add(VerbEntry::regular("resolve", Cognition, VerbGroup::Decide, Some(true), 50));
        self.add(VerbEntry::regular("conclude", Cognition, VerbGroup::Decide, Some(true), 55));
        self.add(VerbEntry::regular("select", Cognition, VerbGroup::Decide, Some(true), 60));
        self.add(VerbEntry::regular("opt", Cognition, VerbGroup::Decide, None, 45));
        self.add(VerbEntry::regular("pick", Cognition, VerbGroup::Decide, Some(true), 70));
        self.add(VerbEntry::regular("prefer", Cognition, VerbGroup::Decide, Some(true), 60));
        self.add(VerbEntry::regular("judge", Cognition, VerbGroup::Decide, Some(true), 60));
        self.add(VerbEntry::regular("evaluate", Cognition, VerbGroup::Decide, Some(true), 55));
        self.add(VerbEntry::regular("assess", Cognition, VerbGroup::Decide, Some(true), 50));

        // PLAN group
        self.add(VerbEntry::regular("plan", Cognition, VerbGroup::Plan, Some(true), 80)
            .with_synonyms(&["intend", "aim", "scheme", "devise", "design", "prepare"]));
        self.add(VerbEntry::regular("intend", Cognition, VerbGroup::Plan, Some(true), 55));
        self.add(VerbEntry::regular("aim", Cognition, VerbGroup::Plan, None, 60));
        self.add(VerbEntry::regular("scheme", Cognition, VerbGroup::Plan, None, 35));
        self.add(VerbEntry::regular("devise", Cognition, VerbGroup::Plan, Some(true), 40));
        self.add(VerbEntry::regular("design", Cognition, VerbGroup::Plan, Some(true), 65));
        self.add(VerbEntry::regular("prepare", Cognition, VerbGroup::Plan, Some(true), 75));
        self.add(VerbEntry::regular("arrange", Cognition, VerbGroup::Plan, Some(true), 60));
        self.add(VerbEntry::regular("organize", Cognition, VerbGroup::Plan, Some(true), 65));
        self.add(VerbEntry::regular("schedule", Cognition, VerbGroup::Plan, Some(true), 55));
        self.add(VerbEntry::regular("coordinate", Cognition, VerbGroup::Plan, Some(true), 50));

        // IMAGINE group
        self.add(VerbEntry::regular("imagine", Cognition, VerbGroup::Imagine, Some(true), 70)
            .with_synonyms(&["envision", "visualize", "dream", "fantasize", "conceive"]));
        self.add(VerbEntry::regular("envision", Cognition, VerbGroup::Imagine, Some(true), 40));
        self.add(VerbEntry::regular("visualize", Cognition, VerbGroup::Imagine, Some(true), 45));
        self.add(VerbEntry::regular("dream", Cognition, VerbGroup::Imagine, None, 65));
        self.add(VerbEntry::regular("fantasize", Cognition, VerbGroup::Imagine, None, 35));
        self.add(VerbEntry::regular("conceive", Cognition, VerbGroup::Imagine, Some(true), 40));
        self.add(VerbEntry::regular("invent", Cognition, VerbGroup::Imagine, Some(true), 55));
        self.add(VerbEntry::regular("picture", Cognition, VerbGroup::Imagine, Some(true), 50));

        // ANALYZE group
        self.add(VerbEntry::regular("analyze", Cognition, VerbGroup::Analyze, Some(true), 60)
            .with_synonyms(&["examine", "investigate", "evaluate", "review", "scrutinize"]));
        self.add(VerbEntry::regular("examine", Cognition, VerbGroup::Analyze, Some(true), 60));
        self.add(VerbEntry::regular("review", Cognition, VerbGroup::Analyze, Some(true), 70));
        self.add(VerbEntry::regular("scrutinize", Cognition, VerbGroup::Analyze, Some(true), 35));
        self.add(VerbEntry::regular("diagnose", Cognition, VerbGroup::Analyze, Some(true), 45));
        self.add(VerbEntry::regular("interpret", Cognition, VerbGroup::Analyze, Some(true), 50));
        self.add(VerbEntry::regular("classify", Cognition, VerbGroup::Analyze, Some(true), 45));
        self.add(VerbEntry::regular("categorize", Cognition, VerbGroup::Analyze, Some(true), 40));
        self.add(VerbEntry::regular("distinguish", Cognition, VerbGroup::Analyze, Some(true), 50));
        self.add(VerbEntry::regular("differentiate", Cognition, VerbGroup::Analyze, Some(true), 40));
    }

    pub(super) fn load_emotion_verbs(&mut self) {
        use FunctionalCategory::Emotion;

        // LOVE group
        self.add(VerbEntry::regular("love", Emotion, VerbGroup::Love, Some(true), 90)
            .with_synonyms(&["adore", "cherish", "treasure", "worship", "idolize"])
            .with_antonyms(&["hate", "detest", "loathe"]));
        self.add(VerbEntry::regular("adore", Emotion, VerbGroup::Love, Some(true), 50));
        self.add(VerbEntry::regular("cherish", Emotion, VerbGroup::Love, Some(true), 45));
        self.add(VerbEntry::regular("treasure", Emotion, VerbGroup::Love, Some(true), 45));
        self.add(VerbEntry::regular("worship", Emotion, VerbGroup::Love, Some(true), 40));
        self.add(VerbEntry::regular("idolize", Emotion, VerbGroup::Love, Some(true), 30));
        self.add(VerbEntry::regular("fancy", Emotion, VerbGroup::Love, Some(true), 40));
        self.add(VerbEntry::regular("admire", Emotion, VerbGroup::Love, Some(true), 55));
        self.add(VerbEntry::regular("respect", Emotion, VerbGroup::Love, Some(true), 65));

        // HATE group
        self.add(VerbEntry::regular("hate", Emotion, VerbGroup::Hate, Some(true), 75)
            .with_synonyms(&["detest", "loathe", "despise", "abhor", "dislike"])
            .with_antonyms(&["love", "adore", "cherish"]));
        self.add(VerbEntry::regular("detest", Emotion, VerbGroup::Hate, Some(true), 40));
        self.add(VerbEntry::regular("loathe", Emotion, VerbGroup::Hate, Some(true), 40));
        self.add(VerbEntry::regular("despise", Emotion, VerbGroup::Hate, Some(true), 45));
        self.add(VerbEntry::regular("abhor", Emotion, VerbGroup::Hate, Some(true), 25));
        self.add(VerbEntry::regular("dislike", Emotion, VerbGroup::Hate, Some(true), 60));
        self.add(VerbEntry::regular("resent", Emotion, VerbGroup::Hate, Some(true), 45));
        self.add(VerbEntry::regular("scorn", Emotion, VerbGroup::Hate, Some(true), 35));
        self.add(VerbEntry::regular("disdain", Emotion, VerbGroup::Hate, Some(true), 30));

        // FEAR group
        self.add(VerbEntry::regular("fear", Emotion, VerbGroup::Fear, Some(true), 70)
            .with_synonyms(&["dread", "worry", "frighten", "scare", "terrify", "alarm"]));
        self.add(VerbEntry::regular("dread", Emotion, VerbGroup::Fear, Some(true), 45));
        self.add(VerbEntry::regular("worry", Emotion, VerbGroup::Fear, None, 75));
        self.add(VerbEntry::regular("frighten", Emotion, VerbGroup::Fear, Some(true), 50));
        self.add(VerbEntry::regular("scare", Emotion, VerbGroup::Fear, Some(true), 60));
        self.add(VerbEntry::regular("terrify", Emotion, VerbGroup::Fear, Some(true), 45));
        self.add(VerbEntry::regular("alarm", Emotion, VerbGroup::Fear, Some(true), 50));
        self.add(VerbEntry::regular("panic", Emotion, VerbGroup::Fear, None, 50));
        self.add(VerbEntry::regular("startle", Emotion, VerbGroup::Fear, Some(true), 40));
        self.add(VerbEntry::regular("intimidate", Emotion, VerbGroup::Fear, Some(true), 45));
        self.add(VerbEntry::regular("threaten", Emotion, VerbGroup::Fear, Some(true), 55));

        // HOPE group
        self.add(VerbEntry::regular("hope", Emotion, VerbGroup::Hope, None, 80)
            .with_synonyms(&["wish", "desire", "long", "yearn", "crave", "aspire"]));
        self.add(VerbEntry::regular("wish", Emotion, VerbGroup::Hope, None, 75));
        self.add(VerbEntry::regular("desire", Emotion, VerbGroup::Hope, Some(true), 55));
        self.add(VerbEntry::regular("long", Emotion, VerbGroup::Hope, None, 50));
        self.add(VerbEntry::regular("yearn", Emotion, VerbGroup::Hope, None, 35));
        self.add(VerbEntry::regular("crave", Emotion, VerbGroup::Hope, Some(true), 45));
        self.add(VerbEntry::regular("aspire", Emotion, VerbGroup::Hope, None, 40));
        self.add(VerbEntry::regular("dream", Emotion, VerbGroup::Hope, None, 65));

        // ENJOY group
        self.add(VerbEntry::regular("enjoy", Emotion, VerbGroup::Enjoy, Some(true), 85)
            .with_synonyms(&["like", "appreciate", "relish", "savor", "delight"]));
        self.add(VerbEntry::regular("like", Emotion, VerbGroup::Enjoy, Some(true), 95));
        self.add(VerbEntry::regular("appreciate", Emotion, VerbGroup::Enjoy, Some(true), 65));
        self.add(VerbEntry::regular("relish", Emotion, VerbGroup::Enjoy, Some(true), 35));
        self.add(VerbEntry::regular("savor", Emotion, VerbGroup::Enjoy, Some(true), 35));
        self.add(VerbEntry::regular("delight", Emotion, VerbGroup::Enjoy, None, 45));
        self.add(VerbEntry::regular("thrill", Emotion, VerbGroup::Enjoy, Some(true), 45));
        self.add(VerbEntry::regular("please", Emotion, VerbGroup::Enjoy, Some(true), 65));
        self.add(VerbEntry::regular("amuse", Emotion, VerbGroup::Enjoy, Some(true), 50));
        self.add(VerbEntry::regular("entertain", Emotion, VerbGroup::Enjoy, Some(true), 55));

        // SUFFER group
        self.add(VerbEntry::regular("suffer", Emotion, VerbGroup::Suffer, None, 65)
            .with_synonyms(&["hurt", "ache", "grieve", "mourn", "lament"]));
        self.add(VerbEntry::irregular("hurt", "hurt", "hurt", Emotion, VerbGroup::Suffer, None, 75));
        self.add(VerbEntry::regular("ache", Emotion, VerbGroup::Suffer, None, 45));
        self.add(VerbEntry::regular("grieve", Emotion, VerbGroup::Suffer, None, 40));
        self.add(VerbEntry::regular("mourn", Emotion, VerbGroup::Suffer, Some(true), 40));
        self.add(VerbEntry::regular("lament", Emotion, VerbGroup::Suffer, Some(true), 30));
        self.add(VerbEntry::regular("regret", Emotion, VerbGroup::Suffer, Some(true), 55));
        self.add(VerbEntry::regular("miss", Emotion, VerbGroup::Suffer, Some(true), 75));
        self.add(VerbEntry::regular("cry", Emotion, VerbGroup::Suffer, None, 65));
        self.add(VerbEntry::irregular("weep", "wept", "wept", Emotion, VerbGroup::Suffer, None, 35));
        self.add(VerbEntry::regular("sob", Emotion, VerbGroup::Suffer, None, 35));

        // SURPRISE group
        self.add(VerbEntry::regular("surprise", Emotion, VerbGroup::Surprise, Some(true), 70)
            .with_synonyms(&["shock", "amaze", "astonish", "astound", "stun", "daze"]));
        self.add(VerbEntry::regular("shock", Emotion, VerbGroup::Surprise, Some(true), 60));
        self.add(VerbEntry::regular("amaze", Emotion, VerbGroup::Surprise, Some(true), 50));
        self.add(VerbEntry::regular("astonish", Emotion, VerbGroup::Surprise, Some(true), 40));
        self.add(VerbEntry::regular("astound", Emotion, VerbGroup::Surprise, Some(true), 35));
        self.add(VerbEntry::regular("stun", Emotion, VerbGroup::Surprise, Some(true), 45));
        self.add(VerbEntry::regular("daze", Emotion, VerbGroup::Surprise, Some(true), 30));
        self.add(VerbEntry::regular("bewilder", Emotion, VerbGroup::Surprise, Some(true), 30));
        self.add(VerbEntry::regular("confuse", Emotion, VerbGroup::Surprise, Some(true), 60));
        self.add(VerbEntry::regular("puzzle", Emotion, VerbGroup::Surprise, Some(true), 45));
        self.add(VerbEntry::regular("perplex", Emotion, VerbGroup::Surprise, Some(true), 30));
        self.add(VerbEntry::regular("baffle", Emotion, VerbGroup::Surprise, Some(true), 35));

        // ANGER group
        self.add(VerbEntry::regular("anger", Emotion, VerbGroup::Anger, Some(true), 55)
            .with_synonyms(&["annoy", "irritate", "enrage", "infuriate", "aggravate", "provoke"]));
        self.add(VerbEntry::regular("annoy", Emotion, VerbGroup::Anger, Some(true), 60));
        self.add(VerbEntry::regular("irritate", Emotion, VerbGroup::Anger, Some(true), 50));
        self.add(VerbEntry::regular("enrage", Emotion, VerbGroup::Anger, Some(true), 35));
        self.add(VerbEntry::regular("infuriate", Emotion, VerbGroup::Anger, Some(true), 35));
        self.add(VerbEntry::regular("aggravate", Emotion, VerbGroup::Anger, Some(true), 45));
        self.add(VerbEntry::regular("provoke", Emotion, VerbGroup::Anger, Some(true), 45));
        self.add(VerbEntry::regular("upset", Emotion, VerbGroup::Anger, Some(true), 70));
        self.add(VerbEntry::regular("frustrate", Emotion, VerbGroup::Anger, Some(true), 55));
        self.add(VerbEntry::regular("offend", Emotion, VerbGroup::Anger, Some(true), 50));
        self.add(VerbEntry::regular("insult", Emotion, VerbGroup::Anger, Some(true), 50));
        self.add(VerbEntry::regular("bother", Emotion, VerbGroup::Anger, Some(true), 65));
        self.add(VerbEntry::regular("disturb", Emotion, VerbGroup::Anger, Some(true), 55));

        // SATISFY group
        self.add(VerbEntry::regular("satisfy", Emotion, VerbGroup::Satisfy, Some(true), 65)
            .with_synonyms(&["please", "content", "gratify", "fulfill"]));
        self.add(VerbEntry::regular("content", Emotion, VerbGroup::Satisfy, Some(true), 40));
        self.add(VerbEntry::regular("gratify", Emotion, VerbGroup::Satisfy, Some(true), 30));
        self.add(VerbEntry::regular("fulfill", Emotion, VerbGroup::Satisfy, Some(true), 50));
        self.add(VerbEntry::regular("comfort", Emotion, VerbGroup::Satisfy, Some(true), 55));
        self.add(VerbEntry::regular("soothe", Emotion, VerbGroup::Satisfy, Some(true), 40));
        self.add(VerbEntry::regular("calm", Emotion, VerbGroup::Satisfy, Some(true), 55));
        self.add(VerbEntry::regular("relax", Emotion, VerbGroup::Satisfy, None, 65));
        self.add(VerbEntry::regular("reassure", Emotion, VerbGroup::Satisfy, Some(true), 45));
        self.add(VerbEntry::regular("relieve", Emotion, VerbGroup::Satisfy, Some(true), 55));

        // Additional emotion verbs
        self.add(VerbEntry::regular("bore", Emotion, VerbGroup::Anger, Some(true), 50));
        self.add(VerbEntry::regular("tire", Emotion, VerbGroup::Suffer, Some(true), 55));
        self.add(VerbEntry::regular("exhaust", Emotion, VerbGroup::Suffer, Some(true), 45));
        self.add(VerbEntry::regular("embarrass", Emotion, VerbGroup::Surprise, Some(true), 50));
        self.add(VerbEntry::regular("humiliate", Emotion, VerbGroup::Anger, Some(true), 40));
        self.add(VerbEntry::regular("shame", Emotion, VerbGroup::Suffer, Some(true), 45));
        self.add(VerbEntry::regular("envy", Emotion, VerbGroup::Hope, Some(true), 40));
        self.add(VerbEntry::regular("pity", Emotion, VerbGroup::Suffer, Some(true), 40));
        self.add(VerbEntry::regular("sympathize", Emotion, VerbGroup::Satisfy, None, 40));
        self.add(VerbEntry::regular("empathize", Emotion, VerbGroup::Satisfy, None, 35));
    }

    pub(super) fn load_physical_verbs(&mut self) {
        use FunctionalCategory::Physical;

        // HIT group
        self.add(VerbEntry::irregular("hit", "hit", "hit", Physical, VerbGroup::Hit, Some(true), 80)
            .with_synonyms(&["strike", "punch", "beat", "slap", "smack", "pound", "hammer"]));
        self.add(VerbEntry::irregular("strike", "struck", "struck", Physical, VerbGroup::Hit, Some(true), 65));
        self.add(VerbEntry::regular("punch", Physical, VerbGroup::Hit, Some(true), 55));
        self.add(VerbEntry::irregular("beat", "beat", "beaten", Physical, VerbGroup::Hit, Some(true), 65));
        self.add(VerbEntry::regular("slap", Physical, VerbGroup::Hit, Some(true), 50));
        self.add(VerbEntry::regular("smack", Physical, VerbGroup::Hit, Some(true), 45));
        self.add(VerbEntry::regular("pound", Physical, VerbGroup::Hit, Some(true), 45));
        self.add(VerbEntry::regular("hammer", Physical, VerbGroup::Hit, Some(true), 45));
        self.add(VerbEntry::regular("knock", Physical, VerbGroup::Hit, Some(true), 60));
        self.add(VerbEntry::regular("tap", Physical, VerbGroup::Hit, Some(true), 55));
        self.add(VerbEntry::regular("whip", Physical, VerbGroup::Hit, Some(true), 40));
        self.add(VerbEntry::regular("bash", Physical, VerbGroup::Hit, Some(true), 40));
        self.add(VerbEntry::regular("bang", Physical, VerbGroup::Hit, Some(true), 50));
        self.add(VerbEntry::regular("bump", Physical, VerbGroup::Hit, Some(true), 50));
        self.add(VerbEntry::regular("crash", Physical, VerbGroup::Hit, None, 55));
        self.add(VerbEntry::regular("collide", Physical, VerbGroup::Hit, None, 40));

        // CUT group
        self.add(VerbEntry::irregular("cut", "cut", "cut", Physical, VerbGroup::Cut, Some(true), 80)
            .with_synonyms(&["slice", "chop", "carve", "trim", "slit", "gash"]));
        self.add(VerbEntry::regular("slice", Physical, VerbGroup::Cut, Some(true), 55));
        self.add(VerbEntry::regular("chop", Physical, VerbGroup::Cut, Some(true), 50));
        self.add(VerbEntry::regular("carve", Physical, VerbGroup::Cut, Some(true), 45));
        self.add(VerbEntry::regular("trim", Physical, VerbGroup::Cut, Some(true), 50));
        self.add(VerbEntry::irregular("slit", "slit", "slit", Physical, VerbGroup::Cut, Some(true), 35));
        self.add(VerbEntry::regular("gash", Physical, VerbGroup::Cut, Some(true), 30));
        self.add(VerbEntry::regular("scratch", Physical, VerbGroup::Cut, Some(true), 55));
        self.add(VerbEntry::regular("scrape", Physical, VerbGroup::Cut, Some(true), 45));
        self.add(VerbEntry::regular("shave", Physical, VerbGroup::Cut, Some(true), 50));
        self.add(VerbEntry::regular("peel", Physical, VerbGroup::Cut, Some(true), 45));
        self.add(VerbEntry::regular("tear", Physical, VerbGroup::Cut, Some(true), 55));
        self.add(VerbEntry::regular("rip", Physical, VerbGroup::Cut, Some(true), 50));
        self.add(VerbEntry::regular("pierce", Physical, VerbGroup::Cut, Some(true), 40));
        self.add(VerbEntry::regular("stab", Physical, VerbGroup::Cut, Some(true), 45));
        self.add(VerbEntry::regular("poke", Physical, VerbGroup::Cut, Some(true), 50));
        self.add(VerbEntry::regular("prick", Physical, VerbGroup::Cut, Some(true), 35));

        // PUSH group
        self.add(VerbEntry::regular("push", Physical, VerbGroup::Push, Some(true), 75)
            .with_synonyms(&["shove", "press", "thrust", "nudge", "force", "squeeze"]));
        self.add(VerbEntry::regular("shove", Physical, VerbGroup::Push, Some(true), 50));
        self.add(VerbEntry::regular("press", Physical, VerbGroup::Push, Some(true), 70));
        self.add(VerbEntry::irregular("thrust", "thrust", "thrust", Physical, VerbGroup::Push, Some(true), 45));
        self.add(VerbEntry::regular("nudge", Physical, VerbGroup::Push, Some(true), 40));
        self.add(VerbEntry::regular("force", Physical, VerbGroup::Push, Some(true), 65));
        self.add(VerbEntry::regular("squeeze", Physical, VerbGroup::Push, Some(true), 55));
        self.add(VerbEntry::regular("compress", Physical, VerbGroup::Push, Some(true), 40));
        self.add(VerbEntry::regular("crush", Physical, VerbGroup::Push, Some(true), 50));
        self.add(VerbEntry::regular("squash", Physical, VerbGroup::Push, Some(true), 40));
        self.add(VerbEntry::regular("flatten", Physical, VerbGroup::Push, Some(true), 40));

        // PULL group
        self.add(VerbEntry::regular("pull", Physical, VerbGroup::Pull, Some(true), 75)
            .with_synonyms(&["drag", "tug", "haul", "yank", "draw", "tow"]));
        self.add(VerbEntry::regular("drag", Physical, VerbGroup::Pull, Some(true), 60));
        self.add(VerbEntry::regular("tug", Physical, VerbGroup::Pull, Some(true), 45));
        self.add(VerbEntry::regular("haul", Physical, VerbGroup::Pull, Some(true), 45));
        self.add(VerbEntry::regular("yank", Physical, VerbGroup::Pull, Some(true), 40));
        self.add(VerbEntry::irregular("draw", "drew", "drawn", Physical, VerbGroup::Pull, Some(true), 70));
        self.add(VerbEntry::regular("tow", Physical, VerbGroup::Pull, Some(true), 40));
        self.add(VerbEntry::regular("extract", Physical, VerbGroup::Pull, Some(true), 45));
        self.add(VerbEntry::regular("pluck", Physical, VerbGroup::Pull, Some(true), 40));
        self.add(VerbEntry::regular("stretch", Physical, VerbGroup::Pull, Some(true), 55));

        // THROW group
        self.add(VerbEntry::irregular("throw", "threw", "thrown", Physical, VerbGroup::Throw, Some(true), 75)
            .with_synonyms(&["toss", "hurl", "cast", "fling", "pitch", "lob"]));
        self.add(VerbEntry::regular("toss", Physical, VerbGroup::Throw, Some(true), 55));
        self.add(VerbEntry::regular("hurl", Physical, VerbGroup::Throw, Some(true), 40));
        self.add(VerbEntry::irregular("cast", "cast", "cast", Physical, VerbGroup::Throw, Some(true), 50));
        self.add(VerbEntry::irregular("fling", "flung", "flung", Physical, VerbGroup::Throw, Some(true), 40));
        self.add(VerbEntry::regular("pitch", Physical, VerbGroup::Throw, Some(true), 50));
        self.add(VerbEntry::regular("lob", Physical, VerbGroup::Throw, Some(true), 30));
        self.add(VerbEntry::regular("launch", Physical, VerbGroup::Throw, Some(true), 55));
        self.add(VerbEntry::regular("fire", Physical, VerbGroup::Throw, Some(true), 65));
        self.add(VerbEntry::irregular("shoot", "shot", "shot", Physical, VerbGroup::Throw, Some(true), 65));

        // CATCH group
        self.add(VerbEntry::irregular("catch", "caught", "caught", Physical, VerbGroup::Catch, Some(true), 75)
            .with_synonyms(&["grab", "seize", "capture", "snatch", "grasp", "clutch"]));
        self.add(VerbEntry::regular("grab", Physical, VerbGroup::Catch, Some(true), 65));
        self.add(VerbEntry::regular("seize", Physical, VerbGroup::Catch, Some(true), 50));
        self.add(VerbEntry::regular("capture", Physical, VerbGroup::Catch, Some(true), 55));
        self.add(VerbEntry::regular("snatch", Physical, VerbGroup::Catch, Some(true), 45));
        self.add(VerbEntry::regular("grasp", Physical, VerbGroup::Catch, Some(true), 50));
        self.add(VerbEntry::regular("clutch", Physical, VerbGroup::Catch, Some(true), 40));
        self.add(VerbEntry::regular("trap", Physical, VerbGroup::Catch, Some(true), 50));

        // HOLD group
        self.add(VerbEntry::irregular("hold", "held", "held", Physical, VerbGroup::Hold, Some(true), 90)
            .with_synonyms(&["grip", "clutch", "grasp", "embrace", "hug", "clasp"]));
        self.add(VerbEntry::regular("grip", Physical, VerbGroup::Hold, Some(true), 50));
        self.add(VerbEntry::regular("embrace", Physical, VerbGroup::Hold, Some(true), 50));
        self.add(VerbEntry::regular("hug", Physical, VerbGroup::Hold, Some(true), 55));
        self.add(VerbEntry::regular("clasp", Physical, VerbGroup::Hold, Some(true), 35));
        self.add(VerbEntry::regular("cling", Physical, VerbGroup::Hold, None, 45));
        self.add(VerbEntry::regular("carry", Physical, VerbGroup::Hold, Some(true), 80));
        self.add(VerbEntry::regular("support", Physical, VerbGroup::Hold, Some(true), 65));
        self.add(VerbEntry::regular("sustain", Physical, VerbGroup::Hold, Some(true), 45));
        self.add(VerbEntry::irregular("bear", "bore", "borne", Physical, VerbGroup::Hold, Some(true), 55));

        // LIFT group
        self.add(VerbEntry::regular("lift", Physical, VerbGroup::Lift, Some(true), 70)
            .with_synonyms(&["raise", "elevate", "hoist", "heave", "pick up"]));
        self.add(VerbEntry::regular("raise", Physical, VerbGroup::Lift, Some(true), 75));
        self.add(VerbEntry::regular("elevate", Physical, VerbGroup::Lift, Some(true), 40));
        self.add(VerbEntry::regular("hoist", Physical, VerbGroup::Lift, Some(true), 35));
        self.add(VerbEntry::regular("heave", Physical, VerbGroup::Lift, Some(true), 35));
        self.add(VerbEntry::regular("lower", Physical, VerbGroup::Lift, Some(true), 55));

        // OPEN/CLOSE groups
        self.add(VerbEntry::regular("open", Physical, VerbGroup::Open, Some(true), 90)
            .with_antonyms(&["close", "shut"]));
        self.add(VerbEntry::regular("unlock", Physical, VerbGroup::Open, Some(true), 55));
        self.add(VerbEntry::regular("unfold", Physical, VerbGroup::Open, Some(true), 45));
        self.add(VerbEntry::regular("uncover", Physical, VerbGroup::Open, Some(true), 45));
        self.add(VerbEntry::regular("unwrap", Physical, VerbGroup::Open, Some(true), 45));
        self.add(VerbEntry::regular("reveal", Physical, VerbGroup::Open, Some(true), 55));
        self.add(VerbEntry::regular("expose", Physical, VerbGroup::Open, Some(true), 50));

        self.add(VerbEntry::regular("close", Physical, VerbGroup::Close, Some(true), 85)
            .with_antonyms(&["open"]));
        self.add(VerbEntry::irregular("shut", "shut", "shut", Physical, VerbGroup::Close, Some(true), 70));
        self.add(VerbEntry::regular("seal", Physical, VerbGroup::Close, Some(true), 50));
        self.add(VerbEntry::regular("lock", Physical, VerbGroup::Close, Some(true), 65));
        self.add(VerbEntry::regular("fasten", Physical, VerbGroup::Close, Some(true), 45));
        self.add(VerbEntry::regular("secure", Physical, VerbGroup::Close, Some(true), 55));
        self.add(VerbEntry::regular("cover", Physical, VerbGroup::Close, Some(true), 70));
        self.add(VerbEntry::regular("wrap", Physical, VerbGroup::Close, Some(true), 55));
        self.add(VerbEntry::regular("fold", Physical, VerbGroup::Close, Some(true), 55));
        self.add(VerbEntry::regular("conceal", Physical, VerbGroup::Close, Some(true), 40));
        self.add(VerbEntry::irregular("hide", "hid", "hidden", Physical, VerbGroup::Close, Some(true), 65));

        // TOUCH group
        self.add(VerbEntry::regular("rub", Physical, VerbGroup::Touch, Some(true), 55));
        self.add(VerbEntry::regular("stroke", Physical, VerbGroup::Touch, Some(true), 45));
        self.add(VerbEntry::regular("pat", Physical, VerbGroup::Touch, Some(true), 45));
        self.add(VerbEntry::regular("caress", Physical, VerbGroup::Touch, Some(true), 35));
        self.add(VerbEntry::regular("massage", Physical, VerbGroup::Touch, Some(true), 45));
        self.add(VerbEntry::regular("tickle", Physical, VerbGroup::Touch, Some(true), 40));
        self.add(VerbEntry::regular("pinch", Physical, VerbGroup::Touch, Some(true), 45));
        self.add(VerbEntry::regular("scratch", Physical, VerbGroup::Touch, Some(true), 55));

        // KICK group
        self.add(VerbEntry::regular("kick", Physical, VerbGroup::Kick, Some(true), 65));
        self.add(VerbEntry::regular("stomp", Physical, VerbGroup::Kick, None, 40));
        self.add(VerbEntry::regular("trample", Physical, VerbGroup::Kick, Some(true), 35));
        self.add(VerbEntry::regular("stamp", Physical, VerbGroup::Kick, None, 45));
        self.add(VerbEntry::regular("step", Physical, VerbGroup::Kick, None, 70));

        // Additional physical verbs
        self.add(VerbEntry::irregular("shake", "shook", "shaken", Physical, VerbGroup::Hit, Some(true), 65));
        self.add(VerbEntry::regular("mix", Physical, VerbGroup::Touch, Some(true), 60));
        self.add(VerbEntry::irregular("stir", "stirred", "stirred", Physical, VerbGroup::Touch, Some(true), 55));
        self.add(VerbEntry::regular("pour", Physical, VerbGroup::Throw, Some(true), 60));
        self.add(VerbEntry::regular("spill", Physical, VerbGroup::Throw, Some(true), 50));
        self.add(VerbEntry::regular("scatter", Physical, VerbGroup::Throw, Some(true), 45));
        self.add(VerbEntry::irregular("spread", "spread", "spread", Physical, VerbGroup::Open, Some(true), 60));
        self.add(VerbEntry::regular("wipe", Physical, VerbGroup::Touch, Some(true), 55));
        self.add(VerbEntry::regular("clean", Physical, VerbGroup::Touch, Some(true), 70));
        self.add(VerbEntry::regular("wash", Physical, VerbGroup::Touch, Some(true), 70));
        self.add(VerbEntry::regular("scrub", Physical, VerbGroup::Touch, Some(true), 45));
        self.add(VerbEntry::regular("polish", Physical, VerbGroup::Touch, Some(true), 40));
        self.add(VerbEntry::regular("brush", Physical, VerbGroup::Touch, Some(true), 55));
        self.add(VerbEntry::regular("comb", Physical, VerbGroup::Touch, Some(true), 45));
    }

    pub(super) fn load_state_verbs(&mut self) {
        use FunctionalCategory::State;

        // BE group
        self.add(VerbEntry::irregular("be", "was", "been", State, VerbGroup::Be, None, 100));
        self.add(VerbEntry::regular("exist", State, VerbGroup::Be, None, 60));
        self.add(VerbEntry::regular("live", State, VerbGroup::Be, None, 85));
        self.add(VerbEntry::regular("survive", State, VerbGroup::Be, None, 55));

        // HAVE group
        self.add(VerbEntry::irregular("have", "had", "had", State, VerbGroup::Have, Some(true), 100));
        self.add(VerbEntry::regular("possess", State, VerbGroup::Have, Some(true), 45));
        self.add(VerbEntry::regular("own", State, VerbGroup::Have, Some(true), 65));
        self.add(VerbEntry::regular("contain", State, VerbGroup::Have, Some(true), 55));
        self.add(VerbEntry::regular("include", State, VerbGroup::Have, Some(true), 65));
        self.add(VerbEntry::regular("comprise", State, VerbGroup::Have, Some(true), 35));
        self.add(VerbEntry::regular("consist", State, VerbGroup::Have, None, 45));

        // SEEM group
        self.add(VerbEntry::regular("seem", State, VerbGroup::Seem, None, 75));
        self.add(VerbEntry::regular("appear", State, VerbGroup::Seem, None, 70));
        self.add(VerbEntry::regular("look", State, VerbGroup::Seem, None, 90));
        self.add(VerbEntry::regular("sound", State, VerbGroup::Seem, None, 70));
        self.add(VerbEntry::regular("smell", State, VerbGroup::Seem, None, 60));
        self.add(VerbEntry::regular("taste", State, VerbGroup::Seem, None, 55));
        self.add(VerbEntry::regular("feel", State, VerbGroup::Seem, None, 85));
        self.add(VerbEntry::regular("prove", State, VerbGroup::Seem, None, 55));

        // REMAIN group
        self.add(VerbEntry::regular("remain", State, VerbGroup::Remain, None, 60));
        self.add(VerbEntry::regular("stay", State, VerbGroup::Remain, None, 80));
        self.add(VerbEntry::regular("persist", State, VerbGroup::Remain, None, 40));
        self.add(VerbEntry::regular("continue", State, VerbGroup::Remain, None, 75));
        self.add(VerbEntry::regular("last", State, VerbGroup::Remain, None, 65));
        self.add(VerbEntry::regular("endure", State, VerbGroup::Remain, None, 40));
        self.add(VerbEntry::irregular("keep", "kept", "kept", State, VerbGroup::Remain, None, 85));

        // BECOME group
        self.add(VerbEntry::irregular("become", "became", "become", State, VerbGroup::Become, None, 85));
        self.add(VerbEntry::irregular("get", "got", "gotten", State, VerbGroup::Become, None, 95));
        self.add(VerbEntry::irregular("grow", "grew", "grown", State, VerbGroup::Become, None, 70));
        self.add(VerbEntry::regular("turn", State, VerbGroup::Become, None, 80));
        self.add(VerbEntry::irregular("go", "went", "gone", State, VerbGroup::Become, None, 90));
        self.add(VerbEntry::regular("develop", State, VerbGroup::Become, None, 60));
        self.add(VerbEntry::regular("evolve", State, VerbGroup::Become, None, 45));
        self.add(VerbEntry::regular("transform", State, VerbGroup::Become, None, 50));
        self.add(VerbEntry::regular("change", State, VerbGroup::Become, None, 80));
        self.add(VerbEntry::regular("convert", State, VerbGroup::Become, Some(true), 45));

        // Additional state verbs
        self.add(VerbEntry::regular("equal", State, VerbGroup::Be, Some(true), 55));
        self.add(VerbEntry::regular("match", State, VerbGroup::Be, Some(true), 60));
        self.add(VerbEntry::regular("resemble", State, VerbGroup::Seem, Some(true), 40));
        self.add(VerbEntry::regular("differ", State, VerbGroup::Be, None, 45));
        self.add(VerbEntry::regular("vary", State, VerbGroup::Become, None, 50));
        self.add(VerbEntry::regular("depend", State, VerbGroup::Be, None, 65));
        self.add(VerbEntry::regular("belong", State, VerbGroup::Have, None, 60));
        self.add(VerbEntry::regular("relate", State, VerbGroup::Be, None, 55));
        self.add(VerbEntry::regular("correspond", State, VerbGroup::Be, None, 40));
        self.add(VerbEntry::regular("fit", State, VerbGroup::Be, None, 65));
        self.add(VerbEntry::regular("suit", State, VerbGroup::Be, Some(true), 50));
        self.add(VerbEntry::regular("deserve", State, VerbGroup::Be, Some(true), 55));
        self.add(VerbEntry::regular("require", State, VerbGroup::Have, Some(true), 65));
        self.add(VerbEntry::regular("involve", State, VerbGroup::Have, Some(true), 60));
        self.add(VerbEntry::regular("matter", State, VerbGroup::Be, None, 65));
        self.add(VerbEntry::regular("count", State, VerbGroup::Be, None, 60));
        self.add(VerbEntry::regular("weigh", State, VerbGroup::Be, None, 50));
        self.add(VerbEntry::regular("cost", State, VerbGroup::Be, Some(true), 65));
        self.add(VerbEntry::regular("measure", State, VerbGroup::Be, Some(true), 50));
    }
}
