# Future Features for Rust Tour

## Learning Modes Vision

The learning mode system is designed to scale with content. While currently limited with only 3 exercises, the infrastructure is built to support a rich learning experience as more exercises are added, especially community-contributed ones.

### Planned Learning Modes (When 50+ Exercises Available)

#### 1. **Guided Mode** (Sequential Learning)
- **Purpose**: Follow the official Rust Book learning path
- **Features**:
  - Exercises in recommended order
  - Chapter-based progression
  - Prerequisites enforced
  - Only vetted/official exercises shown
  - Clear difficulty curve
- **Use Case**: New Rust learners wanting structured education

#### 2. **Explorer Mode** (Free Learning)
- **Purpose**: Discover exercises based on interest
- **Features**:
  - All exercises available (official + community)
  - Filter by tags, author, popularity
  - No order restrictions
  - Discovery-based learning
  - "Surprise me" option
- **Use Case**: Experienced developers exploring Rust features

#### 3. **Focused Mode** (Concept Mastery)
- **Purpose**: Deep dive into specific Rust concepts
- **Features**:
  - Group exercises by concept (ownership, traits, async, etc.)
  - Multiple difficulty levels per concept
  - Concept mastery tracking
  - Related exercises from all sources
  - "Master this concept" paths
- **Use Case**: Developers struggling with specific Rust concepts

#### 4. **Challenge Mode** (Competitive Learning)
- **Purpose**: Test skills with hard problems
- **Features**:
  - Community-rated difficult exercises
  - Time-based challenges
  - Weekly/monthly challenges
  - Leaderboards and achievements
  - Code golf variants
- **Use Case**: Advanced users wanting to push their skills

### Community Features to Support Modes

#### Exercise Metadata Enhancements
- User ratings (difficulty, quality, learning value)
- Concept tags (ownership, lifetimes, traits, etc.)
- Time estimates with user feedback
- Solution quality ratings
- Related exercise suggestions

#### Social Features
- Solution discussions
- Alternative approaches showcase
- Mentorship connections
- "Help me with this" requests
- Community exercise creation tools

#### Adaptive Learning
- Track user performance by concept
- Suggest exercises based on weak areas
- Adaptive difficulty adjustment
- Personalized learning paths
- Progress predictions

### Current State (Beta)
- Infrastructure is built and ready
- Modes provide basic differentiation
- UI clearly indicates beta status
- Prepares users for future richness
- Encourages community contribution

### Contribution Guidelines for Exercise Authors
When creating exercises, consider:
- Clear concept focus
- Accurate difficulty rating
- Good test coverage
- Progressive hints
- Multiple solution approaches
- Real-world applicability

### Technical Infrastructure Ready
- Mode switching system ✓
- Recommendation engine ✓
- Progress tracking ✓
- Exercise metadata structure ✓
- UI components ✓

The platform is designed to grow. The current limited exercise set is just the beginning!