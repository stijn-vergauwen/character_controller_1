# Character controller v1
First: This is not a complete general controller you can just plop in your project.  
I made this project mainly for personal use. Not everything is documented, clean, or even finished.

![image](https://github.com/stijn-vergauwen/character_controller_1/assets/85249104/7f030edf-f875-47a7-baa5-634f3b8377fc)
The default character with a third-person camera moving on a sloped surface. With debug gizmos enabled.

## About
A physics-based character controller, it's pretty general-purpose with behaviour configurations and some optional modules.

This was my first attempt at making a reusable plugin for Bevy. Went pretty well!  
The quality of the code is better than previous projects and I learned a bunch about: removing dependencies, simplifying calculations into clear objects or functions, and quaternions.

Many pieces of the code aren't completely finished but I'm leaving this project here. It's for learning purposes and I believe starting fresh with a next version is best for learning.

### Functionality
- Basic movement, running, and jumping.
- Configuration for character size, speeds, forces etc.
- Input source is decoupled from the character (you decide how to control it).
- Camera is optional and you can spawn in a first-person, third-person, or custom cam component.
- Movement works on slopes and keeps velocity aligned to the input direction.

### How to use
Your project needs to have the 'bevy' & 'bevy_rapier3d' dependencies.

This is how I spawned the character when testing.  
Create a new CharacterSpawner, it's methods should provide enough info on how to use it.

```Rust
let spawn_settings = CharacterSpawnSettings::default();
let character = Character::default();
let character_config = CharacterConfig::default();

// This component is optional, you can also use your own input handling.
let movement_input = PlayerMovementInput::default();

CharacterSpawner::new(spawn_settings)
    .spawn_core(&mut commands, character, character_config)
    .add_body(&mut commands, &mut meshes, &mut materials)
    .add_jumping(&mut commands)
    // .add_camera(&mut commands, build_first_person_camera())
    .add_camera(&mut commands, build_third_person_camera(7.0))
    .add_root_component(&mut commands, movement_input);
```

### Next steps
The things I want to change and improve on the next version:
- Use newtypes to make data more descriptive, e.g. "turn speed" has a very weird value.
- Set the character's mass manually and make the collider densities 0, for consistency.
- Add character interaction, didn't have a clear idea of how that would look with this one.
- Grounded & jump components:
    - Don't make grounded component optional, if the functionality already works why would you not just always use it?
    - Prevent sliding when idle on a slope. snap transform back to prev position or try making the rigidbody sleep.
    - When the grounded check casts a shape, use that shape collision to get the normal directly, the current system causes the character to get stuck a bit on slope edges.
- Crouching component:
    - Use a different system for crouching, either:
        - Resize the collider downwards so character stays grounded.
        - Make alternative to resizing like splitting body in multiple parts and rotating those, which would be more accurate also.
    - No magic numbers in crouching code, move these parameters to a config.
