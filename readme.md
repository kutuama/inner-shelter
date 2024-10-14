/inner-shelter
  /core                  # Domain layer with ECS entities, components, systems
    /components          # ECS components (e.g., Health, Position, InventoryComponent)
    /entities            # ECS entities (Players, NPCs, etc.)
    /systems             # Core gameplay systems (MovementSystem, CombatSystem)
    /services            # Domain services (specific game logic services using ECS)
    
  /application            # Application Layer
    /game                # Game-related orchestration (how systems combine for interactions)
    /interfaces          # Interfaces for integrating systems with infrastructure

  /infrastructure         # Infrastructure Layer
    /bevy                # Bevy ECS engine integration
    /redis               # Redis caching for game state
    /websocket           # WebSocket for real-time communication
    /postgresql          # Database handling persistent data
    /rabbitmq            # Async messaging for non-real-time events

  /client                 # Client (Godot or Bevy front-end)
    /scripts             # Client-side scripts for handling input and UI

  /server                 # Game server logic (focused on ECS with Bevy)
    /game_state          # Uses ECS to manage game state with systems and components

  /service                # Service logic for non-real-time tasks
    /auth                # Authentication services
    /inventory           # Inventory services


brew services start redis
redis-cli