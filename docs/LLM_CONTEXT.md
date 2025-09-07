# LLM Context & Project Management Document

## INSTRUCTIONS FOR LLM

**You are working on a Rust CLI roguelike game called "Vibe". This document contains all context needed to resume work on this project at any time.**

### Essential Context
- **User Profile**: Experienced with TDD but new to Rust and game development
- **Development Approach**: Test-driven development throughout
- **Architecture Goal**: Clean, domain-driven design with platform abstractions
- **Current Phase**: Documentation and initial setup

### Key Documents
1. **GAME_DESIGN.md** - The authoritative specification (human & LLM readable)
2. **LLM_CONTEXT.md** - This document (LLM-focused tracking)

### Your Responsibilities
1. **Always read GAME_DESIGN.md first** when resuming work
2. **Update both documents** as the project evolves
3. **Maintain the TODO tracking** in this document
4. **Follow TDD principles** religiously
5. **Ask clarifying questions** about Rust concepts when needed

## PROJECT STATE

### Current Status: **DESIGN COMPLETION PHASE**
- ✅ Initial game design document created
- ✅ LLM context document created
- ✅ Living World Memory System designed
- ✅ Progressive Combat Discovery System designed
- ✅ Universal Statistical System (D20 + percentage) designed
- ✅ Universal State System (temporal dynamics) designed
- ✅ Infinite Composable Storytelling framework designed
- ✅ **Story Entity Hierarchy System designed**
- ✅ **Procedural World Generation through Stat Contests designed**
- ✅ **Stage-Based Narrative Progression System designed**
- ✅ **Text Generation System (Compressed LM) designed**
- 🔄 **Finalizing core design before implementation**

### Next Immediate Tasks
1. Initialize Rust project with Cargo and Nix
2. Set up project structure according to GAME_DESIGN.md
3. Create basic platform abstraction traits
4. Implement first integration test (character creation)

### Implementation Strategy
**START SIMPLE, BUILD UP**:
1. Basic terminal I/O
2. Character creation only
3. Simple movement on empty map
4. Basic combat with one monster type
5. Expand from there

## PROGRESS TRACKING

### Completed Major Design Systems
- [x] Project planning and architecture design
- [x] Living World Memory & Environmental Archaeology System
- [x] Progressive Combat Discovery & Earthbound-style Humor System
- [x] Universal Statistical System (D20 scale, percentage-based outcomes)
- [x] Universal State System (temporal dynamics, infinite composability)
- [x] Generational Story Continuation & Family Legacy System
- [x] Item Consciousness & Evolution Mechanics
- [x] Class-Specific Information Discovery Mechanics
- [x] Advanced Interface System (non-disruptive popups, searchable logs)
- [x] Post-Game Analysis & SQLite Export System
- [x] **Story Entity Hierarchy System** (Story→Stages→Regions contest system)
- [x] **Procedural World Terraforming** (Story vs Terrain stat contests)
- [x] **Stage-Based Narrative Progression** (regional mastery advancement)
- [x] **Compressed Language Model Text Generation** (contextual, variable-length)

### Outstanding Design Areas to Explore (Optional)
- [ ] **Economy System** (merchant networks, supply/demand, pricing)
- [ ] **Magic System** (spell schools, mana mechanics, magical research)  
- [ ] **Faction & Politics System** (guild relationships, territory control)
- [ ] **Crafting & Item Creation** (recipes, quality levels, experimentation)
- [ ] **Weather & Environmental Systems** (seasons, natural disasters, climate effects)
- [ ] **Transportation & Travel** (mounts, vehicles, fast travel, logistics)
- [ ] **Character Advancement Beyond Stats** (titles, achievements, unlockables)
- [ ] **Endgame Content** (what happens after reaching godlike power?)
- [ ] **Multiplayer Considerations** (how systems would extend to shared world)
- [ ] **Modding & Extension Framework** (how players could add content)

### Current Understanding: Systems Integration
The revolutionary aspect of this design is how ALL systems use the same mathematical foundations:
- **Universal 10-Stat System** (0-20 scale) applies to everything
- **Universal Contest Resolution** (50% baseline + stat differences)
- **Universal State System** (Temporary/Permanent/Toggleable states modify all entities)
- **Mathematical Story Generation** (narratives emerge from statistical interactions)
- **Infinite Composability** (simple rules → infinite unique experiences)

### Current Sprint
**Goal**: Project foundation and basic structure
- [ ] Cargo.toml with dependencies
- [ ] Nix flake configuration
- [ ] Project directory structure
- [ ] Platform abstraction traits
- [ ] Basic entity structs

### Backlog (Priority Order)
1. **Terminal I/O Implementation**
   - crossterm integration
   - basic display and input handling
   - color support

2. **Core Game Loop**
   - main game state
   - turn-based mechanics
   - input → action → update → render cycle

3. **Character System**
   - player classes (start with just Rogue)
   - basic stats system
   - character creation UI

4. **Map & Movement**
   - simple grid-based maps
   - player movement
   - collision detection

5. **Basic Combat**
   - single monster type
   - turn-based combat mechanics
   - simple damage calculation

### Future Phases
- Equipment system
- NPC interactions
- Procedural generation
- Story system
- Polish and optimization

## TECHNICAL DECISIONS LOG

### Architecture Decisions
- **Language**: Rust (user requirement)
- **Display**: Terminal with crossterm crate
- **Testing**: Standard Rust testing + proptest for property-based tests
- **Build**: Nix flakes for reproducible builds
- **Structure**: Layered architecture with platform abstractions

### Dependencies Chosen
```toml
[dependencies]
crossterm = "0.27"      # Terminal manipulation
serde = { version = "1.0", features = ["derive"] }  # Serialization
rand = "0.8"            # Random generation
clap = { version = "4.0", features = ["derive"] }   # CLI parsing

[dev-dependencies]
proptest = "1.0"        # Property-based testing
```

### Key Patterns
- **Entity-Component thinking** but simplified for single-player
- **Command pattern** for all player actions
- **Strategy pattern** for different player classes
- **Abstract Factory** for procedural generation

## TESTING STRATEGY DETAILS

### Test Pyramid
```
    /\
   /  \    E2E Tests (few, critical paths)
  /____\   
 /      \   Integration Tests (moderate, system interactions)
/__________\ Unit Tests (many, individual components)
```

### Critical Test Scenarios
1. **Character Creation Flow**
   - Select class → validate stats → confirm → enter world

2. **Basic Movement**
   - Input key → validate move → update position → redraw

3. **Simple Combat**
   - Encounter monster → enter combat mode → attack/defend → resolution

4. **Equipment Interaction**
   - Find item → pick up → equip → stat changes → combat effects

### Mock Strategy
- **Input/Output**: Mock for deterministic testing
- **Random Generation**: Seeded for reproducible tests
- **Time/Turn System**: Controlled for predictable behavior

## RUST LEARNING NOTES

### Concepts to Explore
- **Ownership & Borrowing**: Critical for game state management
- **Traits**: For platform abstractions and entity behaviors
- **Enums**: Perfect for player classes and game states
- **Error Handling**: Result types for save/load and I/O
- **Modules**: Clean code organization

### Patterns to Use
- **Builder Pattern**: For complex entity creation
- **State Machine**: For game states and AI behaviors
- **Observer Pattern**: For event handling
- **Command Pattern**: For player actions and AI

### Common Pitfalls to Avoid
- Over-engineering early (start simple!)
- Fighting the borrow checker (design with ownership in mind)
- Premature optimization (correctness first)
- Large monolithic functions (keep small and focused)

## DEVELOPMENT WORKFLOW

### Standard Process
1. **Read current state** in this document
2. **Check GAME_DESIGN.md** for specification details
3. **Write tests first** for new functionality
4. **Implement minimum code** to pass tests
5. **Refactor** for clean design
6. **Update both documents** with progress and decisions

### Code Quality Checklist
- [ ] Tests pass and provide good coverage
- [ ] Code follows Rust idioms and conventions
- [ ] Functions are small and single-purpose
- [ ] Names clearly express domain concepts
- [ ] Error handling is comprehensive
- [ ] Documentation explains "why" not just "what"

### Git Workflow
- Small, focused commits
- Clear commit messages explaining the "why"
- Regular pushes to avoid losing work
- Tags for major milestones

## COMMUNICATION GUIDELINES

### When to Ask Questions
- **Rust concepts** that are unclear
- **Game design decisions** that need clarification
- **Architecture choices** with trade-offs
- **Testing approaches** for complex scenarios

### Information to Provide
- **Context** of what you're trying to achieve
- **Specific problem** you're encountering
- **What you've tried** already
- **Relevant code snippets** if applicable

### Preferred Communication Style
- Direct and specific questions
- Code examples when relevant
- References to specific files/functions
- Acknowledgment of user's experience level with different technologies

## UPDATE INSTRUCTIONS

### When to Update This Document
- **Major milestones completed** (update status and progress)
- **Architecture decisions made** (document reasoning)
- **New technical discoveries** (add to learning notes)
- **Workflow changes** (update process documentation)
- **Blocking issues encountered** (track resolution)

### When to Update GAME_DESIGN.md
- **Feature specifications change** (user requests modifications)
- **Technical implementation differs** from original design (keep spec current)
- **New features added** (document the complete system)
- **Architecture evolution** (maintain accuracy)

### Auto-Update Triggers
Update both documents when:
- Completing a major feature
- Starting a new development phase  
- Making significant architectural changes
- User provides new requirements
- Long break in development (before resuming)

## QUICK REFERENCE

### Project Structure (Planned)
```
src/
├── main.rs
├── lib.rs
├── game/          # Core game engine
├── entities/      # Game objects  
├── systems/       # Game mechanics
├── generation/    # Procedural content
├── platform/      # I/O abstractions
├── commands/      # Input handling
└── ui/           # Display rendering
```

### Key Commands (When Implemented)
- Movement: `hjkl` (vim-style)
- Combat: `a`ttack, `d`efend, `s`kill, `f`lee  
- Inventory: `i`nventory, `e`quip, `u`se
- Meta: `?`help, `S`ave, `Q`uit

### Build Commands
```bash
# Development
cargo run
cargo test
cargo check

# With Nix
nix develop        # Enter dev shell
nix build          # Build project
```

---

**Last Updated**: Initial creation - Project setup phase
**Next Review Date**: After project structure implementation
**Current Focus**: Foundation and basic structure setup