# Inner Shelter

Inner Shelter is an open-source game project that leverages modern architectural patterns to ensure scalability, maintainability, and efficiency. This document provides a brief overview of the project structure and the core concepts used, including Onion Architecture and ECS Composition Pattern.

## Project Structure
```
/inner-shelter
  /core                  # Domain layer with ECS entities, components, systems
    /components          # ECS components (e.g., Health, Position, InventoryComponent)
    /entities            # ECS entities (Players, NPCs, etc.)
    /systems             # Core gameplay systems (MovementSystem, CombatSystem)
    /services            # Domain services (specific game logic services using ECS)
  
  /application           # Application Layer
    /game                # Game-related orchestration (how systems combine for interactions)
    /interfaces          # Interfaces for integrating systems with infrastructure

  /infrastructure        # Infrastructure Layer
    /bevy                # Bevy ECS engine integration
    /redis               # Redis caching for game state
    /websocket           # WebSocket for real-time communication
    /cassandra           # Database handling persistent data
    /rabbitmq            # Async messaging for non-real-time events

  /client                # Client (Godot or Bevy front-end)
    /scripts             # Client-side scripts for handling input and UI

  /server                # Game server logic (focused on ECS with Bevy)
    /game_state          # Uses ECS to manage game state with systems and components

  /service               # Service logic for non-real-time tasks
    /auth                # Authentication services
    /inventory           # Inventory services
```

## Core Concepts
### Onion Architecture
Onion Architecture keeps dependencies and responsibilities separated:
- Core: Domain logic including Entities, Components, and Systems.
- Application: Orchestrates game interactions and defines system workflows.
- Infrastructure: Manages external resources (e.g., ECS engine, caching, databases).

### ECS Composition Pattern
ECS is a design pattern used to structure game logic:
- Entities: Game objects like players and NPCs.
- Components: Data attributes of entities (e.g., Health, Position).
- Systems: Logic that acts on entities based on components (e.g., Movement, Combat).