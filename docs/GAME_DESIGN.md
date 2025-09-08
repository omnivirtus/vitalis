# ASCII Roguelike Game Design Document

## Project Vision

**Name**: Vibe (working title)
**Type**: Death-Driven Multi-Generational CLI Roguelike
**Language**: Rust
**Core Innovation**: The Tapestry - A living mathematical weaving system enabling infinite emergent narrative

A revolutionary roguelike where Story Weavers create living Tapestries by connecting all Threads through The Weaver's mathematical relationships, while players witness emerging Patterns and The Seamstress preserves everything across generations, creating unprecedented systemic coherence and emergent storytelling.

## Design Philosophy

### The Tapestry Framework Principle
Every aspect of the game operates through the same weaving mechanics:
- **Thread Properties**: All Threads share the same 10-stat foundation that determines how they weave
- **The Weaving**: All interactions use identical mathematical rules to connect Threads
- **Thread States**: All temporal changes follow the same patterns across all Threads

This creates **infinite composability**—simple weaving rules generate limitless unique Tapestries through emergent complexity.

### Death-Driven Emotional Investment
Unlike traditional roguelikes with quick runs, this game creates **novel-length emotional investment**:
- Playthroughs require 10-120 hours based on story complexity
- Death probability scales 8-25% ensuring meaningful risk
- Multi-generational progression preserves emotional stakes across character deaths
- Player abandonment resolved through the same mathematical framework

---

## I. THE WEAVING SYSTEMS

### Thread Properties
**The Essential Nature of All Threads**

All Threads—players, NPCs, regions, Story Weavers, items, weather systems—possess the same 10 properties (0-20 scale):

**Physical**: Strength, Dexterity, Constitution
**Mental**: Intelligence, Wisdom, Charisma  
**Social**: Connections, Resources, Reputation
**Mystical**: Luck

**Key Insight**: When all Threads share the same properties, complex weaving emerges naturally. A thunderstorm Thread's "Charisma" contests against a player Thread's "Wisdom" to determine if they feel awed or terrified. A sword Thread's "Intelligence" grows through combat experience, eventually offering tactical advice.

### Thread States
**How Threads Change Over Time**

10 states that modify any Thread's weaving capabilities:
**Negative**: Damaged, Corrupted, Stressed, Neglected
**Positive**: Enhanced, Experienced, Connected, Prestigious, Blessed, Adapted

**Key Insight**: States flow between Thread types. A "Corrupted" region Thread makes item Threads found there more likely to gain Corrupted state. An "Experienced" player weapon Thread remembers successful techniques. States create **temporal continuity** that The Seamstress preserves across character deaths.

### The Weaving Engine
**How All Threads Connect Through Mathematics**

**Base Formula**: 50% + (stat_difference × 5%) + d20 + luck_modifier

Whether determining:
- Combat outcomes (Strength vs Constitution)
- Social interactions (Charisma vs Wisdom) 
- Tapestry progression (Player Intelligence vs Story Complexity)
- World generation (Story Weaver stats vs Terrain Thread resistance)
- Abandonment resolution (Story autonomy vs World inertia)

**Key Insight**: Using identical mathematics for all Thread interactions ensures consistent, learnable patterns while enabling unexpected emergent behaviors.

---

## II. ENTITY HIERARCHY & WORLD GENERATION

### Story Entity Hierarchy
**Stories as Protagonists, Not Backdrops**

**Story → Stages → Regions → NPCs → Events**

Stories are entities with their own stats and agency. They **want** to reach completion and actively shape the world to enable their narrative arc.

**Story Generation Process**:
1. **Story Entity Creation**: Random stats determine personality (high Intelligence = complex plot, high Charisma = social focus)
2. **Stage Generation**: Story creates 2-7 stages representing major narrative beats
3. **World Terraforming**: Story contests against terrain to create appropriate regions
4. **Regional Prominence**: Story vs Region contests determine which areas become focal points

**Key Insight**: When stories have agency and statistics, they become living systems that generate appropriate challenges, allies, and obstacles based on mathematical relationships rather than predetermined templates.

### Procedural World Generation Through Stat Contests
**Mathematical Narrative Landscaping with Living Biomes**

The physical world emerges from Story-Terrain contests, creating **living biomes** that interact mathematically with everything:

#### Biome Personalities Through Universal Stats
Each biome type has distinct statistical personalities:

- **Whispering Plains** (Intelligence 8, Charisma 15): Social hubs where rumors travel fast and NPCs gather for complex negotiations
- **Stubborn Mountains** (Strength 18, Constitution 16): Physically demanding regions that test player endurance and equipment durability  
- **Scheming Forests** (Dexterity 14, Wisdom 12): Environments that hide secrets and test player perception while offering stealth advantages
- **Chaotic Deserts** (Constitution 20, Luck 6): Unpredictable survival challenges where preparation battles against randomness
- **Melancholy Swamps** (Intelligence 11, Reputation 4): Isolated regions with forgotten knowledge and social outcasts
- **Bitter Tundra** (Constitution 19, Charisma 3): Hostile environments that bond or break relationships under extreme conditions

#### Dynamic Biome-Story Interactions
Biomes **actively resist or embrace** story progression based on statistical compatibility:
- **High-Intelligence Stories** find academic NPCs and puzzle opportunities in **Whispering Plains**
- **High-Constitution Stories** create epic endurance challenges in **Stubborn Mountains** and **Bitter Tundra**  
- **High-Charisma Stories** turn **Melancholy Swamps** into redemption arcs for outcast communities

#### Weather as Statistical Entities
Weather systems have their own stats and contest against players, biomes, and each other:
- **Thunderstorms** (Strength 16, Charisma 14): Dramatic events that create temporary alliances through shared danger
- **Gentle Rains** (Wisdom 12, Connections 10): Social enhancers that make NPCs more talkative and relationships easier
- **Blizzards** (Constitution 20, Intelligence 4): Survival tests that strip away complexity and focus on basic endurance
- **Magical Auroras** (Intelligence 15, Luck 18): Rare events that enhance discovery and create unexpected opportunities

**Key Insight**: World generation becomes a **living mathematical ecosystem** where biomes, weather, and stories create dynamic relationships that feel purposeful yet remain unpredictably emergent.

---

## III. PROGRESSIVE DISCOVERY SYSTEMS

### Living World Memory System
**Environmental Archaeology**

The world remembers through the Universal State System:
- **Regions** gain states based on player actions (a battlefield becomes "Damaged," a saved village becomes "Blessed")
- **Items** develop consciousness through experience states ("Experienced" weapons remember successful techniques)
- **NPCs** carry generational memory through "Connected" and "Prestigious" states

**Key Insight**: Memory isn't stored data—it's emergent from mathematical state interactions. This creates **authentic continuity** that feels organic rather than programmed.

### Relationship Evolution & Entity Learning
**Dynamic Behavioral Adaptation Through Universal Stats**

Every entity type develops **relationship memory** through statistical interactions:

#### Monster Relationships
**Fear, Respect, and Adaptation**
- **Goblin Tribes**: After multiple encounters, goblins gain "Stressed" states when detecting the player, affecting their Wisdom (tactical decisions) and Dexterity (reaction speed)
- **Wolf Packs**: Repeated interactions reveal pack behavior patterns—high-Intelligence players notice hunting strategies, high-Wisdom players understand territorial boundaries
- **Boss Entities**: Epic encounters create mutual "Experienced" states—both player and monster gain tactical knowledge of each other's capabilities

#### NPC Social Dynamics
**Trust, Reputation, and Economic Relationships**
- **Shopkeepers**: "Connected" state progression unlocks gossip (Intelligence-based information), price discounts (Resources-based negotiations), rare item access (Reputation-based trust)
- **Guild Members**: Relationship depth affects mission complexity—trusted characters receive multi-stage quests, newcomers get simple tasks
- **Authority Figures**: "Prestigious" relationships enable legal protection, political information, or resource access based on player's social stats

#### Terrain Familiarity
**Environmental Mastery Through Experience**
- **Battlefield Terrain**: "Experienced" in specific locations provides combat bonuses—familiar positioning, known hazards, tactical advantages
- **Exploration Areas**: "Adapted" states reduce navigation difficulty, reveal hidden passages, or provide environmental interaction bonuses
- **Hostile Regions**: Unfamiliarity incurs penalties through "Stressed" states, while mastery grants "Enhanced" environmental resistance

**Key Insight**: Relationships aren't dialogue trees—they're **statistical evolution** where every interaction mathematically modifies how entities perceive and respond to each other.

### Progressive Combat Discovery & Earthbound-Style Humor
**Class-Specific Information Revelation with Evolving Comedy**

Each class reveals different aspects of combat through their statistical strengths, with humor that evolves through experience:

#### Rogue (High Dexterity/Intelligence)
- **Discovery**: Spot weak points, optimal positioning, enemy psychology
- **Information Methods**: Stealth eavesdropping, lockpicking hidden diaries, street informant networks
- **Humor Evolution**: 
  - *Early*: "You accidentally step on a twig while sneaking"
  - *Experienced*: "Gary the Goblin has posted 'Wanted' posters of you with surprisingly accurate artistic detail"

#### Warrior (High Strength/Constitution)  
- **Discovery**: Understand damage patterns, defensive opportunities, enemy morale
- **Information Methods**: Hero worship confidences, military honor networks, protective trust relationships
- **Humor Evolution**:
  - *Early*: "Your mighty swing hits the wall instead of the goblin"
  - *Experienced*: "The dragon pauses its rampage to ask for your autograph"

#### Mage (High Intelligence/Wisdom)
- **Discovery**: Recognize magical vulnerabilities, elemental interactions, arcane connections
- **Information Methods**: Academic research, scholarly collaboration, magical divination and scrying  
- **Humor Evolution**:
  - *Early*: "Your fireball fizzles into pretty sparkles"
  - *Experienced*: "The ancient lich interrupts the battle to debate your thesis on temporal magic"

#### Necromancer (High Intelligence/Charisma)
- **Discovery**: Read death probability, undead behavioral patterns, soul resonance
- **Information Methods**: Commune with dead, dark cult networks, fear-based information extraction
- **Humor Evolution**:
  - *Early*: "Your skeleton minion trips over its own femur"
  - *Experienced*: "The tavern's ghost bartender gives you the family discount"

#### Bard (High Charisma/Intelligence)
- **Discovery**: Social dynamics, emotional vulnerabilities, network connections
- **Information Methods**: Charm-based extraction, tavern gossip networks, performance connections
- **Humor Evolution**:
  - *Early*: "Your inspiring ballad accidentally puts the party to sleep"
  - *Experienced*: "Enemy orcs request an encore before resuming hostilities"

#### Barbarian (High Strength/Constitution)
- **Discovery**: Primal instincts, territorial behaviors, survival patterns  
- **Information Methods**: Intimidation-based truth extraction, warrior honor codes, instinct insights
- **Humor Evolution**:
  - *Early*: "Your rage is so intense you attack a tree instead of the enemy"
  - *Experienced*: "Wild animals form an orderly queue to challenge you to honorable combat"

**Key Insight**: Information discovery scales with relevant stats, making character builds feel meaningfully different while maintaining mathematical consistency. Humor creates **emotional attachment** that makes character death genuinely impactful.

### Equipment & Combat Through Statistical Contests
**Turn-Based Mathematical Warfare**

Every combat encounter becomes a **multi-entity statistical contest** where weapons, armor, environment, and combatant psychology all influence outcomes through the Universal Framework:

#### Equipment as Statistical Modifiers
Equipment doesn't just provide flat bonuses—it **modifies the mathematical relationships** during contests:
- **Stubborn Platemail** (Constitution 12, Strength 8): Contests favor endurance over agility, making enemies frustrated and prone to mistakes
- **Whispering Daggers** (Dexterity 15, Intelligence 10): Enable tactical contests that reveal enemy weaknesses through precise strikes
- **Boastful Warhammers** (Strength 18, Charisma 6): Create intimidation contests that affect enemy morale but attract unwanted attention
- **Scholarly Staves** (Intelligence 16, Wisdom 14): Transform combat into knowledge contests where understanding enemy patterns matters more than raw power

#### Dynamic NPC Personalities Through Statistical Psychology
Every NPC possesses **complete statistical personalities** that create emergent social dynamics:

**Shopkeeper Examples**:
- **Greedy Gus** (Resources 16, Charisma 4, Connections 8): High-quality goods at terrible prices, but knows everyone's secrets
- **Anxious Alice** (Intelligence 14, Wisdom 18, Stressed 0.4): Brilliant appraiser who gives great deals to customers who don't stress her out
- **Boastful Bob** (Strength 12, Charisma 16, Prestigious 0.6): Overpriced mundane items but incredible social connections and gossip network

**Guild Master Examples**:
- **Methodical Martha** (Intelligence 18, Constitution 16, Experienced 0.8): Complex multi-stage quests that test planning and persistence  
- **Impulsive Ivan** (Dexterity 15, Luck 12, Stressed 0.3): Dangerous rush jobs with unpredictable rewards and consequences
- **Melancholy Marcus** (Wisdom 16, Charisma 6, Neglected 0.5): Deep philosophical quests about meaning and purpose, often involving redemption

#### Economic Systems Through Resource Contests
Trade negotiations become **statistical contests** between player social stats and NPC economic motivations:
- **High Resources NPCs** vs **Low Resources Players** = Expensive prices but access to rare items
- **High Charisma Players** vs **Low Charisma NPCs** = Charming their way to better deals
- **Connected Players** vs **Isolated NPCs** = Using social networks to pressure pricing

#### Combat Resolution Through Environmental Integration
Combat isn't isolated—it integrates **all nearby statistical entities**:
- **Thunderstorm Combat**: Storm Charisma contests against everyone, creating dramatic moments and temporary alliances
- **Library Battle**: Ambient Intelligence affects spell complexity, making mages stronger but also attracting scholarly NPCs who want to watch
- **Tavern Brawl**: High social density means every action affects multiple NPC relationships simultaneously
- **Mountain Combat**: Terrain Constitution tests everyone's endurance, making long battles favor prepared characters

**Key Insight**: Combat becomes **environmental storytelling** where location, weather, equipment, and NPC psychology create unique narrative moments through pure mathematical interaction.

---

## IV. DEATH-DRIVEN PROGRESSION SYSTEM

### Generational Memory Discovery
**Learning About Your Predecessors Through Retracing**

When creating a new character after death, the world bears **statistical scars** from previous attempts:

#### Environmental Storytelling Through States
**The World Remembers Your Failures and Victories**
- **Cleared Obstacles**: Previous character's successes leave "Enhanced" environmental states—easier navigation, revealed secrets, or helpful modifications
- **Unfinished Business**: Abandoned tasks create "Neglected" states in NPCs/regions—quests become more urgent, relationships deteriorate, problems compound
- **Reputation Echoes**: "Prestigious" or "Corrupted" states attached to your family name affect initial NPC reactions based on predecessor's actions
- **Battle Scars**: Regions where previous characters died gain "Damaged" states—harder terrain, traumatized witnesses, lingering dangers

#### Discovery Through Retracing
**Archaeological Investigation of Your Own Legacy**
- **NPC Testimonies**: Shopkeepers, guild members, and witnesses provide fragmented stories based on their relationship states with your predecessor
- **Environmental Clues**: "Experienced" locations reveal tactical information—optimal positioning, hidden dangers, successful strategies your predecessor discovered
- **Item Inheritance**: "Connected" items remember their previous owner—weapons recall combat techniques, tools retain learned efficiencies
- **Generational Challenges**: Some obstacles can only be overcome by learning from predecessor's attempts—multi-generational puzzles requiring inherited knowledge

#### Relationship Inheritance
**Social Continuity Across Deaths**
- **Family Reputation**: NPCs react to new characters based on accumulated family "Reputation" and "Connections" statistics
- **Inherited Alliances**: "Prestigious" relationships transfer partially—allies provide aid or information, but trust must be rebuilt
- **Inherited Enemies**: "Corrupted" or "Stressed" relationships create immediate hostility—monsters remember your lineage, authorities hold grudges
- **Unresolved Debts**: Economic and social obligations create starting challenges—debts to pay, promises to fulfill, justice to answer for

**Key Insight**: Death isn't reset—it's **generational narrative continuity** where each new character inherits a mathematically-modified world shaped by predecessor's statistical impact.

### Pure Random Character Creation
**Embracing Statistical Destiny**

1. **Random Class Selection**: Equal probability for all classes
2. **Random Backstory Selection**: 20+ backstories per class with different stat weights  
3. **Weighted Random Stats**: Backstory influences but doesn't determine final statistics
4. **Starting States**: Some backstories grant initial Universal States

**Key Insight**: Removing player control over character creation eliminates optimization pressure, forcing adaptation to statistical realities and creating genuine surprise.

### Novel-Length Emotional Investment
**Death Probability Scaling**

**Preparation Quality Determines Survival**:
- **Minimal Preparation**: 25% death probability
- **Partial Preparation**: 18% death probability  
- **Full Preparation**: 12% death probability
- **Exceptional Preparation**: 8% death probability

**Story Length Scaling**:
- **Simple Stories** (Low Intelligence/Complexity): 10-20 hours, 2-3 stages
- **Standard Stories** (Moderate stats): 30-50 hours, 4-5 stages
- **Epic Stories** (High Intelligence/Constitution): 80-120 hours, 6-7 stages

**Key Insight**: Death probability creates **meaningful tension** throughout extended gameplay, while preparation quality rewards player skill without eliminating risk.

### Achievement-Based Progression
**No Time Pressure, Pure Accomplishment**

Stage completion requires achieving specific goals generated by Story-Region contests:
- **Knowledge Goals**: Learn specific information through class-appropriate discovery
- **Relationship Goals**: Establish connections with key NPCs based on social stats
- **Resource Goals**: Obtain items necessary for story progression
- **Challenge Goals**: Overcome obstacles suited to story themes

**Key Insight**: Time pressure destroys emotional investment. Achievement-based progression lets players develop genuine attachment to their characters and world.

---

## V. ABANDONMENT & AUTONOMOUS RESOLUTION

### Universal Contest-Based Abandonment
**Stories Resolve Themselves Mathematically**

When players abandon their character, the game doesn't pause—it resolves autonomously through Story vs World contests:

**Primary Contest**: Story Constitution + Intelligence + Connections vs World Stability + Complexity + Inertia

**Player Impact Modifiers**:
- **Positive momentum** from successful achievements
- **Negative consequences** from failed attempts or damaged relationships
- **Stage progress** influences story's starting position

**Possible Outcomes**:
- **Story Success**: Autonomous completion with positive world changes
- **Story Failure**: Narrative collapse with lasting negative consequences
- **Story Stagnation**: World remains unchanged, story becomes "Neglected" legend

**Key Insight**: Abandonment isn't punishment—it's an additional narrative path where mathematical consistency ensures believable outcomes regardless of player presence.

---

## VI. TEXT GENERATION & INTERFACE SYSTEMS

### Compressed Language Model Integration
**Contextual Narrative Generation**

Text generation draws from:
- **Entity Statistics**: High-Intelligence characters generate complex descriptions
- **Current States**: "Corrupted" entities produce darker text themes
- **Relationship Context**: Social dynamics influence tone and content
- **Historical Memory**: Past events shape present descriptions

**Key Insight**: When text generation operates through the same statistical framework, descriptions feel **systemically consistent** rather than randomly assembled.

### Item Consciousness & Evolution System
**Rarity-Based Sentience Through Mathematical States**

Only special items develop consciousness through accumulated **Experience** states:

#### Sentience Probability
- **Mundane (85%)**: Standard equipment, no personality development
- **Empathic (12%)**: Basic emotional responses, minor stat bonuses through bonding
- **Sentient (2.5%)**: Full personality, evolving relationships, tactical advice
- **Legendary (0.5%)**: Complex psychology, generational memory, reality-bending growth

#### Personality Development Through Universal States
Items gain consciousness through the same mathematical framework:
- **"Experienced" Weapons**: Remember successful combat techniques, offer tactical suggestions
- **"Connected" Tools**: Develop attachment to owners, provide loyalty bonuses
- **"Prestigious" Equipment**: Gain reputation that affects NPC reactions
- **"Corrupted" Items**: Develop malevolent personalities, conflicting goals with owner

#### Cross-Generational Item Memory
Sentient items remember previous owners:
- **Inherited Weapons**: Provide combat knowledge from deceased characters
- **Connected Tools**: Mourn lost owners, gradually warm up to successors  
- **Legendary Items**: Carry complex family histories, multi-generational grudges/loyalties

**Key Insight**: Item consciousness isn't scripted—it emerges from mathematical state accumulation, creating authentic relationships between players and equipment that persist across character deaths.

### Advanced Interface Design  
**Non-Disruptive Information Layering with Environmental Archaeology**

#### Visual Search System
Environmental indicators show discoverable information:
- **`◊`** General searchable areas (previous character actions)
- **`※`** Family history locations (generational memory)  
- **`⚠`** Danger markers (failed attempts, death sites)
- **`✦`** Magical resonance (enchanted item locations)
- **`📜`** Story fragments (narrative progression clues)

#### Advanced Logging & Analysis
- **Contextual Popups**: Information reveals based on class abilities and current stats
- **Searchable Logs**: All generated text stored with statistical metadata
  - Filter by: `[Combat]` `[Discovery]` `[Family]` `[Relationships]`
  - Freeform search: `"grandfather battle"`, `"Gary goblin"`, `"enchanted sword"`
  - Contextual display: Shows 2-3 messages before/after search results
- **Progressive Interface**: UI complexity scales with character Intelligence and Experience states
- **Multi-Generational Continuity**: Interface remembers discoveries across character deaths

#### Post-Game Analysis & SQLite Export
Character death triggers optional comprehensive data export:

**Database Schema**:
- Character genealogy and statistical evolution
- Combat encounters and enemy relationship development  
- Discovery timeline and family history revelations
- Relationship networks and NPC interaction progression
- World state changes and environmental modifications
- Item consciousness development and inheritance patterns

**Analysis Capabilities**:
- Cross-character statistical comparisons
- Family tree visualization with achievement mapping
- World evolution tracking across generations
- Relationship network analysis and social impact measurement

**Key Insight**: The interface becomes part of the Universal State System, evolving based on mathematical interactions. Post-game analysis transforms individual playthroughs into **generational family sagas** with comprehensive data archaeology.

---

## VII. SYSTEM INTERPLAY & EMERGENT BEHAVIORS

### Mathematical Consistency Creates Emergence
**How Simple Rules Generate Complex Experiences**

Because every system uses the same mathematical foundation:

1. **Cross-System State Propagation**: A "Blessed" player might make items more likely to gain "Enhanced" states
2. **Unexpected Entity Relationships**: High-Charisma storms might "befriend" players with matching social stats
3. **Generational Momentum**: "Experienced" locations remember successful strategies, making subsequent characters' attempts easier
4. **Organic Difficulty Scaling**: Challenge emerges from statistical relationships rather than artificial level gates

### Infinite Composability Through Unified Framework
**Why This Design Enables Unprecedented Variety**

Traditional roguelikes create variety through content multiplication (more monsters, more spells, more levels). This design creates variety through **mathematical interaction multiplication**:

- 10 stats × 10 states × entity types = thousands of unique behavioral combinations
- Universal contest system enables any entity to interact meaningfully with any other entity
- Story-driven world generation ensures every playthrough has different environmental logic
- Multi-generational progression creates compound complexity as world states accumulate

**Key Insight**: When every system shares mathematical DNA, the game becomes a **living mathematical ecosystem** capable of generating genuinely novel experiences indefinitely.

---

## VIII. IMPLEMENTATION APPROACH

### Development Philosophy
**Clean Architecture with Domain Focus**

- **Domain-Driven Design**: Code structure reflects mathematical relationships, not technical concerns
- **Test-Driven Development**: Every mathematical relationship validated through comprehensive testing
- **Universal Abstractions**: Single implementations for Universal Statistical System, Contest Resolution, and State Management
- **Emergent Complexity**: Build simple mathematical rules, let complex behaviors emerge

### System Integration Philosophy
**Everything Affects Everything Through Mathematical Relationships**

Rather than isolated game systems, every element continuously influences every other element through shared statistical DNA:

- **Weather systems** modify NPC moods, which affects pricing, which influences player resource strategies, which determines equipment choices, which changes combat outcomes, which alters regional reputation, which shifts story progression
- **Item consciousness** learns from player behavior, which affects equipment recommendations, which influences tactical decisions, which creates combat patterns, which generates environmental changes, which attract specific monster types, which develop into ongoing rivalries
- **Generational memory** shapes NPC attitudes, which determines available quests, which create relationship opportunities, which build social networks, which affect resource access, which enables different playstyles, which lead to unique death scenarios, which modify world states for future characters

### Key Implementation Principles

1. **Mathematical Consistency**: Single implementations for universal systems prevent divergent behavior
2. **State-Driven Logic**: All entity behavior emerges from statistical interactions, not hardcoded rules
3. **Emergent Complexity**: Focus on correct mathematical relationships, let interesting behaviors emerge
4. **Testable Architecture**: Universal framework enables comprehensive property-based testing
5. **Extensible Foundation**: Adding new entity types or interaction patterns requires minimal code changes

---

## IX. SUCCESS METRICS

### Design Validation
**How We Know the System Works**

1. **Mathematical Consistency**: Every interaction follows identical mathematical rules
2. **Emergent Storytelling**: Stories generate appropriate challenges without hardcoded templates
3. **Emotional Investment**: Players develop genuine attachment over 10-120 hour playthroughs
4. **Death Meaningfulness**: Character death feels impactful but not punitive
5. **Infinite Replayability**: Statistical combinations generate genuinely different experiences
6. **System Understanding**: Players can predict outcomes based on learned mathematical relationships
7. **Autonomous Resolution**: Abandoned stories resolve believably through mathematical contests

### Long-Term Vision
**Revolutionary Roguelike Design**

This isn't just another roguelike—it's a demonstration that **mathematical elegance can create unprecedented emergent narrative complexity**. By unifying all game systems through shared statistical DNA, we create:

- **Authentic Emotional Investment** through death-driven multi-generational progression
- **Infinite Emergent Storytelling** through mathematical story-world interactions  
- **Consistent yet Surprising Gameplay** through universal contest resolution
- **Living World Continuity** through persistent mathematical state systems

The goal is proving that **mathematical consistency** rather than content volume is the key to creating truly dynamic, replayable, emotionally engaging roguelike experiences.

---

## X. INTERFACE & USER EXPERIENCE DESIGN

### Vi-Native Modal Interface
**Maximum Vi Compatibility for Terminal Roguelike Experience**

The interface is designed to cause Emacs users' heads to explode through pure vi orthodoxy. Every keybinding, modal interaction, and command follows vi conventions precisely.

#### Terminal Interface Layout
```
┌──────────────────────────────────────────────────────────────────────────────────────────┐
│ ▌ Korthak    ♥  23/27  ⚡ Lv3  ⚔ Armed          ⚠ Hostile   ◐ 47% Explored   Night ☾    ▌│
├──────────────────────────────────────────────────────────────────────────────────────────┤
│ » You hear faint muttering from the east. The goblin seems agitated.              [3/27] │
├─ dungeon.lvl3 ───────────────────────────────────────────────────────────────────────────┤
│██████████████████████████████████████████████████████████████████████████████████████████│
│█▓▓▓▓▓▓▓█░░░░░░░░░░░░░░░░█▒▒▒▒▒▒▒▒▒▒▒█≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈≈█▓▓▓▓▓▓▓▓▓█│
│█▓·····▓█░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒█▒·········▒█≈·······································≈█▓·······▓█│
│█▓·····▓█░▒·············▒█▒·········▒█≈·······································≈█▓·······▓█│
│█▓·····▓█░▒·············▒█▒·········▒█≈···················@···················≈█▓·······▓█│
│█▓·····▓█░▒·············▒█▒·········▒█≈················G······················≈█▓·······▓█│
│██████████████████████████████████████████████████████████████████████████████████████████│
├──────────────────────────────────────────────────────────────────────────────────────────┤
│ -- NORMAL --                                                                     [67,42] │
└──────────────────────────────────────────────────────────────────────────────────────────┘
```

#### Extended ASCII Aesthetic
- **Rich texturing** through varied ASCII shading (█▓▒░≈·)
- **Room differentiation** via material patterns and visual depth
- **Clean symbols** for entities (@ player, G goblin) against textured backgrounds
- **Neovim-inspired** status bars and file-like presentation
- **Responsive layout** that adapts to user's terminal dimensions

### Universal Action Grammar
**Consistent Vi-Style Command Structure**

Every action follows the identical pattern for maximum muscle memory efficiency:

```
[count][which][type][target]
```

#### Pattern Components:
- **Count**: Optional number prefix (3, 5, 10)
- **Which**: Punctuation mark identifying specific item ('  "  ;  ,  /  \  [  ]  -  =)
- **Type**: Single character for action category
- **Target**: Direction or area specification

#### Type Categories:
```
w = weapons          # 'wh = attack left with weapon '
s = scrolls          # "sip = use scroll " on self + adjacent  
k = skills           # ;kiw = use skill ; on self
m = memorized spells # ,mj = cast memorized spell , down
c = consumables      # /cap = use consumable / on adjacent area
e = equipment        # \eh = use equipment \ to left
```

#### Target Specifications:
```
hjkl = directional movement/targeting
iw   = self (inner word - vi text object)
ip   = self + adjacent 8 tiles (inner paragraph)
ap   = adjacent 8 tiles only (a paragraph)
```

### Universal Mark System
**Vi Marks for Everything**

Marks provide instant access to locations, items, spells, and abilities using vi's native mark system:

#### Mark Types:
```
mx    # set mark x (context-dependent: location, item, spell, skill)
gx    # goto mark x (pure vi syntax)
```

#### Mark Categories:
- **Location marks**: Navigate instantly to marked dungeon positions
- **Item marks**: Quick access to specific weapons, consumables, equipment
- **Spell marks**: Memorized spells and scrolls with instant casting
- **Skill marks**: Abilities like stealth, lockpicking, climbing

#### Mark Pool:
Each category uses the same 10 punctuation marks: `'  "  ;  ,  /  \  [  ]  -  =`
Total: 40+ quick-access slots across all categories.

### Modal System
**Pure Vi Modal Philosophy**

#### Normal Mode (Default):
- **Movement**: hjkl with count prefixes (5h, 10j)
- **Actions**: Single-key commands following universal grammar
- **Marks**: Setting and jumping to marks
- **Repeat**: `.` repeats last action (sacred vi command)

#### Insert Mode:
- **Naming**: Items, locations, save files, character notes  
- **Text entry**: Any situation requiring human-readable labels
- **Exit**: ESC returns to normal mode

#### Ex Command Mode:
```
:help, :h         # help system and documentation
:quit, :q         # quit game  
:save, :w         # save game with optional naming
:messages         # show persistent message history
:marks            # display all current marks
:map              # keymap customization (full vi mapping support)
:set              # configuration and preferences
```

### Message System
**Neovim-Style Notifications with Persistent History**

#### Message Display:
```
│ ℹ You hear faint muttering from the east.                                          [3/27] │
│ ⚠ The goblin looks angry!                                                          [2/27] │ 
│ ✓ You picked up 15 gold pieces.                                                    [1/27] │
```

#### Message Navigation:
```
<C-n>/<C-p>       # next/previous message (neovim-style)
<C-u>/<C-d>       # page up/down through message history
/pattern          # search forward in messages (vi search)
?pattern          # search backward in messages  
n/N               # next/previous search result
```

#### Message Features:
- **Persistent history**: No clearing - messages become part of narrative log
- **Searchable**: Full vi-style search through entire adventure history
- **Contextual**: Information, warnings, success indicators with appropriate icons
- **Turn tracking**: Every message timestamped with turn counter

### Customization System
**Full Vi Mapping Support**

#### Custom Keymaps:
```
:map lh 'wh<CR>              # map lh to attack left with primary weapon
:map ;; ;miw<CR>             # map ;; to cast ; spell on self  
:map <F1> :help<CR>          # map function keys
```

#### Leader Key Support:
```
:let mapleader = "\\"        # set leader key
:map <leader>h :help<CR>     # \h for help
:map <leader>s :save<CR>     # \s for save
```

#### Configuration:
```
:set cmdheight=2             # show 2 message lines
:set verbose=1               # show debug messages  
:set quiet                   # suppress info messages
```

### Key Design Principles:

1. **Vi Orthodoxy**: Every keybinding and interaction follows vi conventions precisely
2. **Universal Grammar**: Single pattern for all actions eliminates mental overhead
3. **Muscle Memory**: Consistent syntax enables automatic responses during complex situations
4. **Mark Integration**: Vi's mark system extended to all game entities for instant access
5. **Modal Purity**: Clean separation between action (normal), text entry (insert), and configuration (ex)
6. **Terminal Native**: Designed specifically for terminal interface with extended ASCII enhancement
7. **Customizable**: Full vi-style mapping system for personal workflow optimization

**Key Insight**: By embracing vi's interface philosophy completely, the roguelike becomes an extension of the editor experience rather than a separate application, creating unprecedented efficiency for vi users while maintaining the classic terminal roguelike aesthetic through rich extended ASCII presentation.

---

*This design represents a complete reimagining of roguelike architecture, where mathematical elegance replaces content multiplication as the source of infinite variety and emergent complexity.*
