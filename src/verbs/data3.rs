//! Complete English Verb Database - Part 3
//! Transfer, Creation, Destruction, Control, Possession, Social verbs

use super::{VerbDatabase, VerbEntry, FunctionalCategory, VerbGroup};

impl VerbDatabase {
    pub(super) fn load_transfer_verbs(&mut self) {
        use FunctionalCategory::Transfer;

        // GIVE group
        self.add(VerbEntry::irregular("give", "gave", "given", Transfer, VerbGroup::Give, Some(true), 95)
            .with_synonyms(&["donate", "grant", "offer", "provide", "supply", "present"])
            .with_antonyms(&["take", "receive"]));
        self.add(VerbEntry::regular("donate", Transfer, VerbGroup::Give, Some(true), 50));
        self.add(VerbEntry::regular("grant", Transfer, VerbGroup::Give, Some(true), 50));
        self.add(VerbEntry::regular("offer", Transfer, VerbGroup::Give, Some(true), 75));
        self.add(VerbEntry::regular("provide", Transfer, VerbGroup::Give, Some(true), 75));
        self.add(VerbEntry::regular("supply", Transfer, VerbGroup::Give, Some(true), 55));
        self.add(VerbEntry::regular("present", Transfer, VerbGroup::Give, Some(true), 60));
        self.add(VerbEntry::regular("hand", Transfer, VerbGroup::Give, Some(true), 70));
        self.add(VerbEntry::regular("pass", Transfer, VerbGroup::Give, Some(true), 75));
        self.add(VerbEntry::regular("contribute", Transfer, VerbGroup::Give, Some(true), 55));
        self.add(VerbEntry::regular("award", Transfer, VerbGroup::Give, Some(true), 50));
        self.add(VerbEntry::regular("assign", Transfer, VerbGroup::Give, Some(true), 55));
        self.add(VerbEntry::regular("allocate", Transfer, VerbGroup::Give, Some(true), 40));
        self.add(VerbEntry::regular("distribute", Transfer, VerbGroup::Give, Some(true), 50));
        self.add(VerbEntry::irregular("pay", "paid", "paid", Transfer, VerbGroup::Give, Some(true), 85));
        self.add(VerbEntry::regular("repay", Transfer, VerbGroup::Give, Some(true), 45));
        self.add(VerbEntry::regular("reimburse", Transfer, VerbGroup::Give, Some(true), 35));
        self.add(VerbEntry::regular("compensate", Transfer, VerbGroup::Give, Some(true), 40));
        self.add(VerbEntry::regular("reward", Transfer, VerbGroup::Give, Some(true), 50));

        // TAKE group
        self.add(VerbEntry::irregular("take", "took", "taken", Transfer, VerbGroup::Take, Some(true), 98)
            .with_synonyms(&["grab", "seize", "acquire", "obtain", "get", "accept"])
            .with_antonyms(&["give", "provide"]));
        self.add(VerbEntry::regular("acquire", Transfer, VerbGroup::Take, Some(true), 55));
        self.add(VerbEntry::regular("obtain", Transfer, VerbGroup::Take, Some(true), 55));
        self.add(VerbEntry::regular("accept", Transfer, VerbGroup::Take, Some(true), 70));
        self.add(VerbEntry::regular("claim", Transfer, VerbGroup::Take, Some(true), 60));
        self.add(VerbEntry::regular("collect", Transfer, VerbGroup::Take, Some(true), 65));
        self.add(VerbEntry::regular("gather", Transfer, VerbGroup::Take, Some(true), 55));
        self.add(VerbEntry::regular("fetch", Transfer, VerbGroup::Take, Some(true), 45));
        self.add(VerbEntry::regular("retrieve", Transfer, VerbGroup::Take, Some(true), 45));
        self.add(VerbEntry::regular("adopt", Transfer, VerbGroup::Take, Some(true), 50));
        self.add(VerbEntry::regular("assume", Transfer, VerbGroup::Take, Some(true), 55));
        self.add(VerbEntry::regular("inherit", Transfer, VerbGroup::Take, Some(true), 45));

        // SEND group
        self.add(VerbEntry::irregular("send", "sent", "sent", Transfer, VerbGroup::Send, Some(true), 85)
            .with_synonyms(&["deliver", "ship", "transmit", "dispatch", "forward", "mail"]));
        self.add(VerbEntry::regular("deliver", Transfer, VerbGroup::Send, Some(true), 65));
        self.add(VerbEntry::regular("ship", Transfer, VerbGroup::Send, Some(true), 55));
        self.add(VerbEntry::regular("transmit", Transfer, VerbGroup::Send, Some(true), 45));
        self.add(VerbEntry::regular("dispatch", Transfer, VerbGroup::Send, Some(true), 40));
        self.add(VerbEntry::regular("forward", Transfer, VerbGroup::Send, Some(true), 55));
        self.add(VerbEntry::regular("mail", Transfer, VerbGroup::Send, Some(true), 50));
        self.add(VerbEntry::regular("post", Transfer, VerbGroup::Send, Some(true), 55));
        self.add(VerbEntry::regular("export", Transfer, VerbGroup::Send, Some(true), 45));
        self.add(VerbEntry::regular("transfer", Transfer, VerbGroup::Send, Some(true), 60));
        self.add(VerbEntry::regular("relay", Transfer, VerbGroup::Send, Some(true), 40));
        self.add(VerbEntry::regular("convey", Transfer, VerbGroup::Send, Some(true), 40));
        self.add(VerbEntry::regular("transport", Transfer, VerbGroup::Send, Some(true), 50));

        // RECEIVE group
        self.add(VerbEntry::regular("receive", Transfer, VerbGroup::Receive, Some(true), 75)
            .with_synonyms(&["get", "obtain", "accept", "acquire", "gain"]));
        self.add(VerbEntry::regular("gain", Transfer, VerbGroup::Receive, Some(true), 60));
        self.add(VerbEntry::regular("earn", Transfer, VerbGroup::Receive, Some(true), 65));
        self.add(VerbEntry::irregular("win", "won", "won", Transfer, VerbGroup::Receive, Some(true), 75));
        self.add(VerbEntry::regular("import", Transfer, VerbGroup::Receive, Some(true), 45));

        // LEND/BORROW group
        self.add(VerbEntry::irregular("lend", "lent", "lent", Transfer, VerbGroup::Lend, Some(true), 55)
            .with_antonyms(&["borrow"]));
        self.add(VerbEntry::regular("loan", Transfer, VerbGroup::Lend, Some(true), 50));
        self.add(VerbEntry::regular("advance", Transfer, VerbGroup::Lend, Some(true), 45));

        self.add(VerbEntry::regular("borrow", Transfer, VerbGroup::Borrow, Some(true), 55)
            .with_antonyms(&["lend"]));
        self.add(VerbEntry::regular("rent", Transfer, VerbGroup::Borrow, Some(true), 55));
        self.add(VerbEntry::regular("lease", Transfer, VerbGroup::Borrow, Some(true), 40));
        self.add(VerbEntry::regular("hire", Transfer, VerbGroup::Borrow, Some(true), 55));

        // STEAL group
        self.add(VerbEntry::irregular("steal", "stole", "stolen", Transfer, VerbGroup::Steal, Some(true), 55)
            .with_synonyms(&["rob", "pilfer", "swipe", "snatch"]));
        self.add(VerbEntry::regular("rob", Transfer, VerbGroup::Steal, Some(true), 50));
        self.add(VerbEntry::regular("pilfer", Transfer, VerbGroup::Steal, Some(true), 25));
        self.add(VerbEntry::regular("swipe", Transfer, VerbGroup::Steal, Some(true), 35));
        self.add(VerbEntry::regular("embezzle", Transfer, VerbGroup::Steal, Some(true), 25));

        // RETURN group
        self.add(VerbEntry::regular("return", Transfer, VerbGroup::Return, Some(true), 75));
        self.add(VerbEntry::regular("restore", Transfer, VerbGroup::Return, Some(true), 50));
        self.add(VerbEntry::regular("replace", Transfer, VerbGroup::Return, Some(true), 65));
        self.add(VerbEntry::regular("refund", Transfer, VerbGroup::Return, Some(true), 40));

        // Additional transfer
        self.add(VerbEntry::regular("exchange", Transfer, VerbGroup::Give, Some(true), 55));
        self.add(VerbEntry::regular("swap", Transfer, VerbGroup::Give, Some(true), 50));
        self.add(VerbEntry::regular("trade", Transfer, VerbGroup::Give, Some(true), 55));
        self.add(VerbEntry::irregular("sell", "sold", "sold", Transfer, VerbGroup::Give, Some(true), 80));
        self.add(VerbEntry::irregular("buy", "bought", "bought", Transfer, VerbGroup::Take, Some(true), 85));
        self.add(VerbEntry::regular("purchase", Transfer, VerbGroup::Take, Some(true), 55));
        self.add(VerbEntry::irregular("spend", "spent", "spent", Transfer, VerbGroup::Give, Some(true), 75));
        self.add(VerbEntry::regular("invest", Transfer, VerbGroup::Give, Some(true), 55));
        self.add(VerbEntry::regular("deposit", Transfer, VerbGroup::Give, Some(true), 45));
        self.add(VerbEntry::irregular("withdraw", "withdrew", "withdrawn", Transfer, VerbGroup::Take, Some(true), 50));
    }

    pub(super) fn load_creation_verbs(&mut self) {
        use FunctionalCategory::Creation;

        // MAKE group
        self.add(VerbEntry::irregular("make", "made", "made", Creation, VerbGroup::Make, Some(true), 98)
            .with_synonyms(&["produce", "manufacture", "fabricate", "construct", "form", "fashion"]));
        self.add(VerbEntry::regular("produce", Creation, VerbGroup::Make, Some(true), 70));
        self.add(VerbEntry::regular("manufacture", Creation, VerbGroup::Make, Some(true), 45));
        self.add(VerbEntry::regular("fabricate", Creation, VerbGroup::Make, Some(true), 35));
        self.add(VerbEntry::regular("form", Creation, VerbGroup::Make, Some(true), 65));
        self.add(VerbEntry::regular("fashion", Creation, VerbGroup::Make, Some(true), 35));
        self.add(VerbEntry::regular("craft", Creation, VerbGroup::Make, Some(true), 45));
        self.add(VerbEntry::regular("forge", Creation, VerbGroup::Make, Some(true), 40));
        self.add(VerbEntry::regular("mold", Creation, VerbGroup::Make, Some(true), 40));
        self.add(VerbEntry::regular("shape", Creation, VerbGroup::Make, Some(true), 55));

        // CREATE group
        self.add(VerbEntry::regular("create", Creation, VerbGroup::Create, Some(true), 85)
            .with_synonyms(&["invent", "design", "develop", "originate", "generate", "devise"]));
        self.add(VerbEntry::regular("invent", Creation, VerbGroup::Create, Some(true), 55));
        self.add(VerbEntry::regular("design", Creation, VerbGroup::Create, Some(true), 70));
        self.add(VerbEntry::regular("develop", Creation, VerbGroup::Create, Some(true), 75));
        self.add(VerbEntry::regular("originate", Creation, VerbGroup::Create, Some(true), 35));
        self.add(VerbEntry::regular("generate", Creation, VerbGroup::Create, Some(true), 60));
        self.add(VerbEntry::regular("devise", Creation, VerbGroup::Create, Some(true), 40));
        self.add(VerbEntry::regular("formulate", Creation, VerbGroup::Create, Some(true), 40));
        self.add(VerbEntry::regular("establish", Creation, VerbGroup::Create, Some(true), 60));
        self.add(VerbEntry::irregular("found", "founded", "founded", Creation, VerbGroup::Create, Some(true), 50));
        self.add(VerbEntry::regular("institute", Creation, VerbGroup::Create, Some(true), 35));
        self.add(VerbEntry::regular("pioneer", Creation, VerbGroup::Create, Some(true), 35));
        self.add(VerbEntry::regular("innovate", Creation, VerbGroup::Create, None, 40));

        // BUILD group
        self.add(VerbEntry::irregular("build", "built", "built", Creation, VerbGroup::Build, Some(true), 80)
            .with_synonyms(&["construct", "erect", "assemble", "put together"]));
        self.add(VerbEntry::regular("construct", Creation, VerbGroup::Build, Some(true), 55));
        self.add(VerbEntry::regular("erect", Creation, VerbGroup::Build, Some(true), 35));
        self.add(VerbEntry::regular("assemble", Creation, VerbGroup::Build, Some(true), 50));
        self.add(VerbEntry::regular("install", Creation, VerbGroup::Build, Some(true), 55));
        self.add(VerbEntry::regular("set up", Creation, VerbGroup::Build, Some(true), 70));

        // WRITE group
        self.add(VerbEntry::irregular("write", "wrote", "written", Creation, VerbGroup::Write, Some(true), 90)
            .with_synonyms(&["compose", "author", "draft", "pen", "record"]));
        self.add(VerbEntry::regular("compose", Creation, VerbGroup::Write, Some(true), 50));
        self.add(VerbEntry::regular("author", Creation, VerbGroup::Write, Some(true), 35));
        self.add(VerbEntry::regular("draft", Creation, VerbGroup::Write, Some(true), 50));
        self.add(VerbEntry::regular("pen", Creation, VerbGroup::Write, Some(true), 30));
        self.add(VerbEntry::regular("record", Creation, VerbGroup::Write, Some(true), 65));
        self.add(VerbEntry::regular("document", Creation, VerbGroup::Write, Some(true), 50));
        self.add(VerbEntry::regular("inscribe", Creation, VerbGroup::Write, Some(true), 30));
        self.add(VerbEntry::regular("note", Creation, VerbGroup::Write, Some(true), 60));
        self.add(VerbEntry::regular("type", Creation, VerbGroup::Write, Some(true), 60));
        self.add(VerbEntry::regular("print", Creation, VerbGroup::Write, Some(true), 65));
        self.add(VerbEntry::regular("publish", Creation, VerbGroup::Write, Some(true), 55));
        self.add(VerbEntry::regular("edit", Creation, VerbGroup::Write, Some(true), 55));

        // DRAW group
        self.add(VerbEntry::irregular("draw", "drew", "drawn", Creation, VerbGroup::Draw, Some(true), 70)
            .with_synonyms(&["sketch", "paint", "illustrate", "depict", "portray"]));
        self.add(VerbEntry::regular("sketch", Creation, VerbGroup::Draw, Some(true), 45));
        self.add(VerbEntry::regular("paint", Creation, VerbGroup::Draw, Some(true), 60));
        self.add(VerbEntry::regular("illustrate", Creation, VerbGroup::Draw, Some(true), 45));
        self.add(VerbEntry::regular("depict", Creation, VerbGroup::Draw, Some(true), 40));
        self.add(VerbEntry::regular("portray", Creation, VerbGroup::Draw, Some(true), 40));
        self.add(VerbEntry::regular("trace", Creation, VerbGroup::Draw, Some(true), 50));
        self.add(VerbEntry::regular("outline", Creation, VerbGroup::Draw, Some(true), 50));
        self.add(VerbEntry::regular("color", Creation, VerbGroup::Draw, Some(true), 55));
        self.add(VerbEntry::regular("photograph", Creation, VerbGroup::Draw, Some(true), 50));
        self.add(VerbEntry::regular("film", Creation, VerbGroup::Draw, Some(true), 50));
        self.add(VerbEntry::regular("sculpt", Creation, VerbGroup::Draw, Some(true), 35));
        self.add(VerbEntry::regular("carve", Creation, VerbGroup::Draw, Some(true), 45));

        // COOK group
        self.add(VerbEntry::regular("cook", Creation, VerbGroup::Cook, Some(true), 75)
            .with_synonyms(&["bake", "fry", "roast", "grill", "boil", "prepare"]));
        self.add(VerbEntry::regular("bake", Creation, VerbGroup::Cook, Some(true), 55));
        self.add(VerbEntry::regular("fry", Creation, VerbGroup::Cook, Some(true), 50));
        self.add(VerbEntry::regular("roast", Creation, VerbGroup::Cook, Some(true), 45));
        self.add(VerbEntry::regular("grill", Creation, VerbGroup::Cook, Some(true), 45));
        self.add(VerbEntry::regular("boil", Creation, VerbGroup::Cook, Some(true), 50));
        self.add(VerbEntry::regular("steam", Creation, VerbGroup::Cook, Some(true), 40));
        self.add(VerbEntry::regular("simmer", Creation, VerbGroup::Cook, Some(true), 35));
        self.add(VerbEntry::regular("brew", Creation, VerbGroup::Cook, Some(true), 40));
        self.add(VerbEntry::regular("blend", Creation, VerbGroup::Cook, Some(true), 45));

        // GROW group
        self.add(VerbEntry::irregular("grow", "grew", "grown", Creation, VerbGroup::Grow, Some(true), 75)
            .with_synonyms(&["cultivate", "raise", "plant", "breed", "farm"]));
        self.add(VerbEntry::regular("cultivate", Creation, VerbGroup::Grow, Some(true), 40));
        self.add(VerbEntry::regular("raise", Creation, VerbGroup::Grow, Some(true), 65));
        self.add(VerbEntry::regular("plant", Creation, VerbGroup::Grow, Some(true), 55));
        self.add(VerbEntry::irregular("breed", "bred", "bred", Creation, VerbGroup::Grow, Some(true), 45));
        self.add(VerbEntry::regular("farm", Creation, VerbGroup::Grow, Some(true), 45));
        self.add(VerbEntry::regular("harvest", Creation, VerbGroup::Grow, Some(true), 45));
        self.add(VerbEntry::irregular("sow", "sowed", "sown", Creation, VerbGroup::Grow, Some(true), 35));
        self.add(VerbEntry::regular("water", Creation, VerbGroup::Grow, Some(true), 55));
        self.add(VerbEntry::regular("fertilize", Creation, VerbGroup::Grow, Some(true), 35));
        self.add(VerbEntry::regular("nurture", Creation, VerbGroup::Grow, Some(true), 40));
    }

    pub(super) fn load_destruction_verbs(&mut self) {
        use FunctionalCategory::Destruction;

        // DESTROY group
        self.add(VerbEntry::regular("destroy", Destruction, VerbGroup::Destroy, Some(true), 65)
            .with_synonyms(&["demolish", "wreck", "annihilate", "devastate", "ruin", "obliterate"]));
        self.add(VerbEntry::regular("demolish", Destruction, VerbGroup::Destroy, Some(true), 40));
        self.add(VerbEntry::regular("wreck", Destruction, VerbGroup::Destroy, Some(true), 45));
        self.add(VerbEntry::regular("annihilate", Destruction, VerbGroup::Destroy, Some(true), 30));
        self.add(VerbEntry::regular("devastate", Destruction, VerbGroup::Destroy, Some(true), 40));
        self.add(VerbEntry::regular("ruin", Destruction, VerbGroup::Destroy, Some(true), 50));
        self.add(VerbEntry::regular("obliterate", Destruction, VerbGroup::Destroy, Some(true), 30));
        self.add(VerbEntry::regular("decimate", Destruction, VerbGroup::Destroy, Some(true), 30));
        self.add(VerbEntry::regular("ravage", Destruction, VerbGroup::Destroy, Some(true), 30));
        self.add(VerbEntry::regular("waste", Destruction, VerbGroup::Destroy, Some(true), 60));
        self.add(VerbEntry::regular("spoil", Destruction, VerbGroup::Destroy, Some(true), 50));

        // BREAK group
        self.add(VerbEntry::irregular("break", "broke", "broken", Destruction, VerbGroup::Break, Some(true), 80)
            .with_synonyms(&["shatter", "crack", "smash", "fracture", "snap", "split"]));
        self.add(VerbEntry::regular("shatter", Destruction, VerbGroup::Break, Some(true), 45));
        self.add(VerbEntry::regular("crack", Destruction, VerbGroup::Break, Some(true), 55));
        self.add(VerbEntry::regular("smash", Destruction, VerbGroup::Break, Some(true), 50));
        self.add(VerbEntry::regular("fracture", Destruction, VerbGroup::Break, Some(true), 35));
        self.add(VerbEntry::regular("snap", Destruction, VerbGroup::Break, Some(true), 50));
        self.add(VerbEntry::irregular("split", "split", "split", Destruction, VerbGroup::Break, Some(true), 55));
        self.add(VerbEntry::regular("crush", Destruction, VerbGroup::Break, Some(true), 50));
        self.add(VerbEntry::regular("crumble", Destruction, VerbGroup::Break, None, 40));
        self.add(VerbEntry::regular("collapse", Destruction, VerbGroup::Break, None, 55));
        self.add(VerbEntry::regular("burst", Destruction, VerbGroup::Break, None, 50));
        self.add(VerbEntry::regular("explode", Destruction, VerbGroup::Break, None, 50));

        // KILL group
        self.add(VerbEntry::regular("kill", Destruction, VerbGroup::Kill, Some(true), 70)
            .with_synonyms(&["murder", "slay", "execute", "assassinate", "eliminate", "exterminate"]));
        self.add(VerbEntry::regular("murder", Destruction, VerbGroup::Kill, Some(true), 50));
        self.add(VerbEntry::irregular("slay", "slew", "slain", Destruction, VerbGroup::Kill, Some(true), 35));
        self.add(VerbEntry::regular("execute", Destruction, VerbGroup::Kill, Some(true), 45));
        self.add(VerbEntry::regular("assassinate", Destruction, VerbGroup::Kill, Some(true), 35));
        self.add(VerbEntry::regular("eliminate", Destruction, VerbGroup::Kill, Some(true), 50));
        self.add(VerbEntry::regular("exterminate", Destruction, VerbGroup::Kill, Some(true), 30));
        self.add(VerbEntry::regular("slaughter", Destruction, VerbGroup::Kill, Some(true), 35));
        self.add(VerbEntry::regular("massacre", Destruction, VerbGroup::Kill, Some(true), 25));

        // DAMAGE group
        self.add(VerbEntry::regular("damage", Destruction, VerbGroup::Damage, Some(true), 65)
            .with_synonyms(&["harm", "injure", "hurt", "impair", "mar"]));
        self.add(VerbEntry::regular("harm", Destruction, VerbGroup::Damage, Some(true), 55));
        self.add(VerbEntry::regular("injure", Destruction, VerbGroup::Damage, Some(true), 55));
        self.add(VerbEntry::irregular("hurt", "hurt", "hurt", Destruction, VerbGroup::Damage, Some(true), 75));
        self.add(VerbEntry::regular("impair", Destruction, VerbGroup::Damage, Some(true), 35));
        self.add(VerbEntry::regular("mar", Destruction, VerbGroup::Damage, Some(true), 25));
        self.add(VerbEntry::regular("wound", Destruction, VerbGroup::Damage, Some(true), 50));
        self.add(VerbEntry::regular("maim", Destruction, VerbGroup::Damage, Some(true), 25));
        self.add(VerbEntry::regular("cripple", Destruction, VerbGroup::Damage, Some(true), 30));
        self.add(VerbEntry::regular("disable", Destruction, VerbGroup::Damage, Some(true), 40));
        self.add(VerbEntry::regular("weaken", Destruction, VerbGroup::Damage, Some(true), 50));

        // BURN group
        self.add(VerbEntry::irregular("burn", "burned", "burned", Destruction, VerbGroup::Burn, Some(true), 65)
            .with_synonyms(&["incinerate", "scorch", "char", "singe", "ignite"]));
        self.add(VerbEntry::regular("incinerate", Destruction, VerbGroup::Burn, Some(true), 30));
        self.add(VerbEntry::regular("scorch", Destruction, VerbGroup::Burn, Some(true), 35));
        self.add(VerbEntry::regular("char", Destruction, VerbGroup::Burn, Some(true), 25));
        self.add(VerbEntry::regular("singe", Destruction, VerbGroup::Burn, Some(true), 30));
        self.add(VerbEntry::regular("ignite", Destruction, VerbGroup::Burn, Some(true), 40));
        self.add(VerbEntry::regular("kindle", Destruction, VerbGroup::Burn, Some(true), 30));
        self.add(VerbEntry::regular("blaze", Destruction, VerbGroup::Burn, None, 35));
        self.add(VerbEntry::regular("melt", Destruction, VerbGroup::Burn, Some(true), 50));
        self.add(VerbEntry::regular("dissolve", Destruction, VerbGroup::Burn, Some(true), 45));
        self.add(VerbEntry::regular("corrode", Destruction, VerbGroup::Burn, Some(true), 30));
        self.add(VerbEntry::regular("rust", Destruction, VerbGroup::Burn, None, 40));
        self.add(VerbEntry::regular("rot", Destruction, VerbGroup::Burn, None, 40));
        self.add(VerbEntry::regular("decay", Destruction, VerbGroup::Burn, None, 40));
        self.add(VerbEntry::regular("decompose", Destruction, VerbGroup::Burn, None, 35));

        // ERASE group
        self.add(VerbEntry::regular("erase", Destruction, VerbGroup::Erase, Some(true), 55)
            .with_synonyms(&["delete", "remove", "wipe", "clear", "eliminate"]));
        self.add(VerbEntry::regular("delete", Destruction, VerbGroup::Erase, Some(true), 60));
        self.add(VerbEntry::regular("remove", Destruction, VerbGroup::Erase, Some(true), 75));
        self.add(VerbEntry::regular("wipe", Destruction, VerbGroup::Erase, Some(true), 55));
        self.add(VerbEntry::regular("clear", Destruction, VerbGroup::Erase, Some(true), 65));
        self.add(VerbEntry::regular("discard", Destruction, VerbGroup::Erase, Some(true), 45));
        self.add(VerbEntry::regular("dispose", Destruction, VerbGroup::Erase, Some(true), 45));
        self.add(VerbEntry::regular("dump", Destruction, VerbGroup::Erase, Some(true), 50));
        self.add(VerbEntry::regular("abolish", Destruction, VerbGroup::Erase, Some(true), 35));
        self.add(VerbEntry::regular("cancel", Destruction, VerbGroup::Erase, Some(true), 60));
        self.add(VerbEntry::regular("undo", Destruction, VerbGroup::Erase, Some(true), 50));
    }

    pub(super) fn load_control_verbs(&mut self) {
        use FunctionalCategory::Control;

        // CONTROL group
        self.add(VerbEntry::regular("control", Control, VerbGroup::ControlGroup, Some(true), 75)
            .with_synonyms(&["manage", "handle", "operate", "regulate", "dominate", "command"]));
        self.add(VerbEntry::regular("manage", Control, VerbGroup::ControlGroup, Some(true), 70));
        self.add(VerbEntry::regular("handle", Control, VerbGroup::ControlGroup, Some(true), 70));
        self.add(VerbEntry::regular("operate", Control, VerbGroup::ControlGroup, Some(true), 60));
        self.add(VerbEntry::regular("regulate", Control, VerbGroup::ControlGroup, Some(true), 45));
        self.add(VerbEntry::regular("dominate", Control, VerbGroup::ControlGroup, Some(true), 45));
        self.add(VerbEntry::regular("manipulate", Control, VerbGroup::ControlGroup, Some(true), 45));
        self.add(VerbEntry::regular("administer", Control, VerbGroup::ControlGroup, Some(true), 45));
        self.add(VerbEntry::regular("run", Control, VerbGroup::ControlGroup, Some(true), 85));
        self.add(VerbEntry::regular("conduct", Control, VerbGroup::ControlGroup, Some(true), 55));
        self.add(VerbEntry::regular("execute", Control, VerbGroup::ControlGroup, Some(true), 50));
        self.add(VerbEntry::regular("implement", Control, VerbGroup::ControlGroup, Some(true), 55));
        self.add(VerbEntry::regular("enforce", Control, VerbGroup::ControlGroup, Some(true), 45));
        self.add(VerbEntry::regular("maintain", Control, VerbGroup::ControlGroup, Some(true), 60));

        // LEAD group
        self.add(VerbEntry::irregular("lead", "led", "led", Control, VerbGroup::Lead, Some(true), 75)
            .with_synonyms(&["guide", "direct", "conduct", "head", "captain", "steer"]));
        self.add(VerbEntry::regular("guide", Control, VerbGroup::Lead, Some(true), 60));
        self.add(VerbEntry::regular("direct", Control, VerbGroup::Lead, Some(true), 65));
        self.add(VerbEntry::regular("head", Control, VerbGroup::Lead, Some(true), 55));
        self.add(VerbEntry::regular("captain", Control, VerbGroup::Lead, Some(true), 35));
        self.add(VerbEntry::regular("steer", Control, VerbGroup::Lead, Some(true), 45));
        self.add(VerbEntry::regular("pilot", Control, VerbGroup::Lead, Some(true), 45));
        self.add(VerbEntry::regular("navigate", Control, VerbGroup::Lead, Some(true), 45));
        self.add(VerbEntry::regular("chair", Control, VerbGroup::Lead, Some(true), 40));
        self.add(VerbEntry::regular("preside", Control, VerbGroup::Lead, None, 35));
        self.add(VerbEntry::regular("pioneer", Control, VerbGroup::Lead, Some(true), 35));

        // GOVERN group
        self.add(VerbEntry::regular("govern", Control, VerbGroup::Govern, Some(true), 50)
            .with_synonyms(&["rule", "reign", "administer", "dictate", "preside"]));
        self.add(VerbEntry::regular("rule", Control, VerbGroup::Govern, Some(true), 55));
        self.add(VerbEntry::regular("reign", Control, VerbGroup::Govern, None, 40));
        self.add(VerbEntry::regular("dictate", Control, VerbGroup::Govern, Some(true), 40));
        self.add(VerbEntry::regular("legislate", Control, VerbGroup::Govern, None, 30));

        // SUPERVISE group
        self.add(VerbEntry::regular("supervise", Control, VerbGroup::Supervise, Some(true), 50)
            .with_synonyms(&["oversee", "monitor", "watch", "inspect", "check"]));
        self.add(VerbEntry::regular("oversee", Control, VerbGroup::Supervise, Some(true), 45));
        self.add(VerbEntry::regular("monitor", Control, VerbGroup::Supervise, Some(true), 55));
        self.add(VerbEntry::regular("inspect", Control, VerbGroup::Supervise, Some(true), 50));
        self.add(VerbEntry::regular("check", Control, VerbGroup::Supervise, Some(true), 80));
        self.add(VerbEntry::regular("audit", Control, VerbGroup::Supervise, Some(true), 40));
        self.add(VerbEntry::regular("review", Control, VerbGroup::Supervise, Some(true), 65));
        self.add(VerbEntry::regular("verify", Control, VerbGroup::Supervise, Some(true), 50));
        self.add(VerbEntry::regular("evaluate", Control, VerbGroup::Supervise, Some(true), 55));

        // INFLUENCE group
        self.add(VerbEntry::regular("influence", Control, VerbGroup::Influence, Some(true), 55)
            .with_synonyms(&["affect", "impact", "sway", "shape", "mold"]));
        self.add(VerbEntry::regular("affect", Control, VerbGroup::Influence, Some(true), 65));
        self.add(VerbEntry::regular("impact", Control, VerbGroup::Influence, Some(true), 55));
        self.add(VerbEntry::regular("sway", Control, VerbGroup::Influence, Some(true), 35));
        self.add(VerbEntry::regular("shape", Control, VerbGroup::Influence, Some(true), 55));
        self.add(VerbEntry::regular("mold", Control, VerbGroup::Influence, Some(true), 40));
        self.add(VerbEntry::regular("determine", Control, VerbGroup::Influence, Some(true), 60));
        self.add(VerbEntry::regular("decide", Control, VerbGroup::Influence, Some(true), 75));
        self.add(VerbEntry::regular("dictate", Control, VerbGroup::Influence, Some(true), 40));
    }

    pub(super) fn load_possession_verbs(&mut self) {
        use FunctionalCategory::Possession;

        // OWN group
        self.add(VerbEntry::regular("own", Possession, VerbGroup::Own, Some(true), 70)
            .with_synonyms(&["possess", "hold", "have", "retain"]));
        self.add(VerbEntry::regular("possess", Possession, VerbGroup::Own, Some(true), 50));
        self.add(VerbEntry::regular("retain", Possession, VerbGroup::Own, Some(true), 45));

        // ACQUIRE group
        self.add(VerbEntry::regular("acquire", Possession, VerbGroup::Acquire, Some(true), 55)
            .with_synonyms(&["obtain", "gain", "get", "attain", "secure", "procure"]));
        self.add(VerbEntry::regular("obtain", Possession, VerbGroup::Acquire, Some(true), 55));
        self.add(VerbEntry::regular("gain", Possession, VerbGroup::Acquire, Some(true), 60));
        self.add(VerbEntry::regular("attain", Possession, VerbGroup::Acquire, Some(true), 40));
        self.add(VerbEntry::regular("secure", Possession, VerbGroup::Acquire, Some(true), 55));
        self.add(VerbEntry::regular("procure", Possession, VerbGroup::Acquire, Some(true), 30));
        self.add(VerbEntry::regular("accumulate", Possession, VerbGroup::Acquire, Some(true), 40));
        self.add(VerbEntry::regular("amass", Possession, VerbGroup::Acquire, Some(true), 30));
        self.add(VerbEntry::regular("hoard", Possession, VerbGroup::Acquire, Some(true), 30));

        // LOSE group
        self.add(VerbEntry::irregular("lose", "lost", "lost", Possession, VerbGroup::Lose, Some(true), 80)
            .with_synonyms(&["misplace", "forfeit", "surrender", "sacrifice"]));
        self.add(VerbEntry::regular("misplace", Possession, VerbGroup::Lose, Some(true), 40));
        self.add(VerbEntry::regular("forfeit", Possession, VerbGroup::Lose, Some(true), 35));
        self.add(VerbEntry::regular("surrender", Possession, VerbGroup::Lose, Some(true), 45));
        self.add(VerbEntry::regular("sacrifice", Possession, VerbGroup::Lose, Some(true), 45));
        self.add(VerbEntry::regular("relinquish", Possession, VerbGroup::Lose, Some(true), 30));
        self.add(VerbEntry::regular("abandon", Possession, VerbGroup::Lose, Some(true), 50));
        self.add(VerbEntry::irregular("forsake", "forsook", "forsaken", Possession, VerbGroup::Lose, Some(true), 25));

        // KEEP group
        self.add(VerbEntry::irregular("keep", "kept", "kept", Possession, VerbGroup::Keep, Some(true), 90)
            .with_synonyms(&["retain", "maintain", "preserve", "save", "store"]));
        self.add(VerbEntry::regular("maintain", Possession, VerbGroup::Keep, Some(true), 60));
        self.add(VerbEntry::regular("preserve", Possession, VerbGroup::Keep, Some(true), 50));
        self.add(VerbEntry::regular("save", Possession, VerbGroup::Keep, Some(true), 80));
        self.add(VerbEntry::regular("store", Possession, VerbGroup::Keep, Some(true), 60));
        self.add(VerbEntry::regular("reserve", Possession, VerbGroup::Keep, Some(true), 50));
        self.add(VerbEntry::regular("conserve", Possession, VerbGroup::Keep, Some(true), 40));
        self.add(VerbEntry::regular("protect", Possession, VerbGroup::Keep, Some(true), 65));
        self.add(VerbEntry::regular("guard", Possession, VerbGroup::Keep, Some(true), 50));
        self.add(VerbEntry::regular("defend", Possession, VerbGroup::Keep, Some(true), 55));
        self.add(VerbEntry::regular("shelter", Possession, VerbGroup::Keep, Some(true), 40));

        // SHARE group
        self.add(VerbEntry::regular("share", Possession, VerbGroup::Share, Some(true), 75)
            .with_synonyms(&["distribute", "divide", "split", "allocate"]));
        self.add(VerbEntry::regular("distribute", Possession, VerbGroup::Share, Some(true), 50));
        self.add(VerbEntry::regular("divide", Possession, VerbGroup::Share, Some(true), 60));
        self.add(VerbEntry::irregular("split", "split", "split", Possession, VerbGroup::Share, Some(true), 55));
        self.add(VerbEntry::regular("allocate", Possession, VerbGroup::Share, Some(true), 40));
        self.add(VerbEntry::regular("portion", Possession, VerbGroup::Share, Some(true), 30));
        self.add(VerbEntry::regular("ration", Possession, VerbGroup::Share, Some(true), 30));
    }

    pub(super) fn load_social_verbs(&mut self) {
        use FunctionalCategory::Social;

        // MEET group
        self.add(VerbEntry::irregular("meet", "met", "met", Social, VerbGroup::Meet, Some(true), 85)
            .with_synonyms(&["encounter", "greet", "welcome", "join"]));
        self.add(VerbEntry::regular("encounter", Social, VerbGroup::Meet, Some(true), 50));
        self.add(VerbEntry::regular("greet", Social, VerbGroup::Meet, Some(true), 55));
        self.add(VerbEntry::regular("welcome", Social, VerbGroup::Meet, Some(true), 60));
        self.add(VerbEntry::regular("join", Social, VerbGroup::Meet, Some(true), 75));
        self.add(VerbEntry::regular("gather", Social, VerbGroup::Meet, None, 55));
        self.add(VerbEntry::regular("assemble", Social, VerbGroup::Meet, None, 45));
        self.add(VerbEntry::regular("convene", Social, VerbGroup::Meet, None, 35));
        self.add(VerbEntry::regular("congregate", Social, VerbGroup::Meet, None, 25));

        // HELP group
        self.add(VerbEntry::regular("help", Social, VerbGroup::Help, Some(true), 90)
            .with_synonyms(&["assist", "aid", "support", "serve", "benefit"]));
        self.add(VerbEntry::regular("assist", Social, VerbGroup::Help, Some(true), 60));
        self.add(VerbEntry::regular("aid", Social, VerbGroup::Help, Some(true), 50));
        self.add(VerbEntry::regular("support", Social, VerbGroup::Help, Some(true), 70));
        self.add(VerbEntry::regular("serve", Social, VerbGroup::Help, Some(true), 70));
        self.add(VerbEntry::regular("benefit", Social, VerbGroup::Help, Some(true), 50));
        self.add(VerbEntry::regular("rescue", Social, VerbGroup::Help, Some(true), 50));
        self.add(VerbEntry::regular("save", Social, VerbGroup::Help, Some(true), 75));
        self.add(VerbEntry::regular("protect", Social, VerbGroup::Help, Some(true), 65));
        self.add(VerbEntry::regular("defend", Social, VerbGroup::Help, Some(true), 55));
        self.add(VerbEntry::regular("facilitate", Social, VerbGroup::Help, Some(true), 45));
        self.add(VerbEntry::regular("enable", Social, VerbGroup::Help, Some(true), 50));

        // FIGHT group
        self.add(VerbEntry::irregular("fight", "fought", "fought", Social, VerbGroup::Fight, None, 70)
            .with_synonyms(&["battle", "combat", "struggle", "wrestle", "clash"]));
        self.add(VerbEntry::regular("battle", Social, VerbGroup::Fight, None, 50));
        self.add(VerbEntry::regular("combat", Social, VerbGroup::Fight, Some(true), 40));
        self.add(VerbEntry::regular("struggle", Social, VerbGroup::Fight, None, 55));
        self.add(VerbEntry::regular("wrestle", Social, VerbGroup::Fight, None, 40));
        self.add(VerbEntry::regular("clash", Social, VerbGroup::Fight, None, 40));
        self.add(VerbEntry::regular("confront", Social, VerbGroup::Fight, Some(true), 50));
        self.add(VerbEntry::regular("challenge", Social, VerbGroup::Fight, Some(true), 55));
        self.add(VerbEntry::regular("attack", Social, VerbGroup::Fight, Some(true), 60));
        self.add(VerbEntry::regular("assault", Social, VerbGroup::Fight, Some(true), 40));
        self.add(VerbEntry::regular("invade", Social, VerbGroup::Fight, Some(true), 40));
        self.add(VerbEntry::regular("defend", Social, VerbGroup::Fight, Some(true), 55));
        self.add(VerbEntry::regular("protect", Social, VerbGroup::Fight, Some(true), 65));

        // COOPERATE group
        self.add(VerbEntry::regular("cooperate", Social, VerbGroup::Cooperate, None, 50)
            .with_synonyms(&["collaborate", "participate", "contribute", "unite"]));
        self.add(VerbEntry::regular("collaborate", Social, VerbGroup::Cooperate, None, 45));
        self.add(VerbEntry::regular("participate", Social, VerbGroup::Cooperate, None, 55));
        self.add(VerbEntry::regular("contribute", Social, VerbGroup::Cooperate, None, 55));
        self.add(VerbEntry::regular("unite", Social, VerbGroup::Cooperate, None, 45));
        self.add(VerbEntry::regular("team", Social, VerbGroup::Cooperate, None, 50));
        self.add(VerbEntry::regular("partner", Social, VerbGroup::Cooperate, None, 45));
        self.add(VerbEntry::regular("ally", Social, VerbGroup::Cooperate, None, 40));

        // COMPETE group
        self.add(VerbEntry::regular("compete", Social, VerbGroup::Compete, None, 55)
            .with_synonyms(&["rival", "contest", "vie", "race"]));
        self.add(VerbEntry::regular("rival", Social, VerbGroup::Compete, Some(true), 40));
        self.add(VerbEntry::regular("contest", Social, VerbGroup::Compete, Some(true), 45));
        self.add(VerbEntry::regular("vie", Social, VerbGroup::Compete, None, 30));
        self.add(VerbEntry::regular("race", Social, VerbGroup::Compete, None, 60));
        self.add(VerbEntry::regular("oppose", Social, VerbGroup::Compete, Some(true), 50));

        // FOLLOW group
        self.add(VerbEntry::regular("follow", Social, VerbGroup::Follow, Some(true), 80)
            .with_synonyms(&["pursue", "chase", "track", "trail", "shadow"]));
        self.add(VerbEntry::regular("pursue", Social, VerbGroup::Follow, Some(true), 55));
        self.add(VerbEntry::regular("chase", Social, VerbGroup::Follow, Some(true), 60));
        self.add(VerbEntry::regular("track", Social, VerbGroup::Follow, Some(true), 55));
        self.add(VerbEntry::regular("trail", Social, VerbGroup::Follow, Some(true), 45));
        self.add(VerbEntry::regular("shadow", Social, VerbGroup::Follow, Some(true), 35));
        self.add(VerbEntry::regular("stalk", Social, VerbGroup::Follow, Some(true), 40));
        self.add(VerbEntry::regular("accompany", Social, VerbGroup::Follow, Some(true), 45));
        self.add(VerbEntry::regular("escort", Social, VerbGroup::Follow, Some(true), 40));

        // OBEY group
        self.add(VerbEntry::regular("obey", Social, VerbGroup::Obey, Some(true), 55)
            .with_synonyms(&["comply", "submit", "yield", "conform", "adhere"])
            .with_antonyms(&["disobey", "resist", "rebel"]));
        self.add(VerbEntry::regular("comply", Social, VerbGroup::Obey, None, 45));
        self.add(VerbEntry::regular("submit", Social, VerbGroup::Obey, None, 50));
        self.add(VerbEntry::regular("yield", Social, VerbGroup::Obey, None, 45));
        self.add(VerbEntry::regular("conform", Social, VerbGroup::Obey, None, 45));
        self.add(VerbEntry::regular("adhere", Social, VerbGroup::Obey, None, 40));
        self.add(VerbEntry::regular("respect", Social, VerbGroup::Obey, Some(true), 65));
        self.add(VerbEntry::regular("honor", Social, VerbGroup::Obey, Some(true), 50));

        // RESIST group
        self.add(VerbEntry::regular("resist", Social, VerbGroup::Resist, Some(true), 55)
            .with_synonyms(&["oppose", "defy", "rebel", "reject", "refuse"])
            .with_antonyms(&["obey", "comply", "submit"]));
        self.add(VerbEntry::regular("oppose", Social, VerbGroup::Resist, Some(true), 50));
        self.add(VerbEntry::regular("defy", Social, VerbGroup::Resist, Some(true), 45));
        self.add(VerbEntry::regular("rebel", Social, VerbGroup::Resist, None, 45));
        self.add(VerbEntry::regular("reject", Social, VerbGroup::Resist, Some(true), 55));
        self.add(VerbEntry::regular("refuse", Social, VerbGroup::Resist, Some(true), 60));
        self.add(VerbEntry::regular("protest", Social, VerbGroup::Resist, Some(true), 50));
        self.add(VerbEntry::regular("object", Social, VerbGroup::Resist, None, 55));
        self.add(VerbEntry::regular("disobey", Social, VerbGroup::Resist, Some(true), 40));
        self.add(VerbEntry::regular("violate", Social, VerbGroup::Resist, Some(true), 45));
        self.add(VerbEntry::irregular("withstand", "withstood", "withstood", Social, VerbGroup::Resist, Some(true), 40));
    }
}
