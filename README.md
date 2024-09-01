# Space Invaders!
> _...or Space Invader? Given there's only one?_
  
https://github.com/user-attachments/assets/99fd3fe4-e330-4a9a-8945-a2988952c840

## Motivation
This was originally going to be a simple game engine, but I realised that would take more time than I'm ready to give to this project right now, so Space Invaders it is!

## Internals
Uses `raylib` for rendering and `bevy_ecs` for the ECS.  
  
All `Drawable`s are rendered, where the idea is that each drawable returns a texture with what they want drawn,
which is then rendered on the main window at the entity's coordinates.
