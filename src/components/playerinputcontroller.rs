extern crate sdl2;

use crate::prefabs;
use crate::prefabs::Player;
use crate::resources::TexturesContainer;
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Scancode;
use std::rc::Rc;

// TODO: We could make this a trait with default implementations.
impl Player {
    pub fn control(
        &mut self,
        tick_counter: u128,
        keyboard_state: &KeyboardState,
        texture_container: &TexturesContainer,
    ) {
        const PLAYER_SPEED: i32 = 3;
        const TICKS_EACH_SPRITE: u128 = 7; // Only powers of two minus one
        const SCALE_DIFF_FACTOR: f32 = 1.01;

        let goup = keyboard_state.is_scancode_pressed(Scancode::L);
        let godown = keyboard_state.is_scancode_pressed(Scancode::K);
        let goleft = keyboard_state.is_scancode_pressed(Scancode::J);
        let goright = keyboard_state.is_scancode_pressed(Scancode::Semicolon);

        let horizontal_movement = goleft ^ goright;
        let vertical_movement = goup ^ godown;
        let old_state = self.state;

        // TODO: When you're moving both horizontally and vertically the player goes
        // faster, because both motions are combined and gives a weird effect.
        // We have to normalize the speeds.

        if horizontal_movement {
            let direction = if goleft { -1 } else { 1 };
            self.transform.x += direction * PLAYER_SPEED;
            self.state = if goleft {
                prefabs::PlayerState::MovingLeft
            } else {
                prefabs::PlayerState::MovingRight
            };
        }

        if vertical_movement {
            let direction = if goup { -1 } else { 1 };
            self.transform.y += direction * PLAYER_SPEED;
            self.state = if goup {
                prefabs::PlayerState::MovingUp
            } else {
                prefabs::PlayerState::MovingDown
            };
        }

        if !horizontal_movement && !vertical_movement {
            self.state = match self.state {
                prefabs::PlayerState::MovingLeft => prefabs::PlayerState::IdleLeft,
                prefabs::PlayerState::MovingRight => prefabs::PlayerState::IdleRight,
                prefabs::PlayerState::MovingDown => prefabs::PlayerState::IdleDown,
                prefabs::PlayerState::MovingUp => prefabs::PlayerState::IdleUp,
                _ => self.state,
            }
        }

        let instant_sprite_change = self.state != old_state;

        if instant_sprite_change || tick_counter & TICKS_EACH_SPRITE == 0 {
            macro_rules! update_moving_sprite {
                ($sprite:ident) => {
                    if Rc::ptr_eq(&self.sprite.track, &texture_container.$sprite) {
                        self.sprite.index += 1;
                        self.sprite.index %= self.sprite.track.len();
                    } else {
                        self.sprite.track = texture_container.$sprite.clone();
                    }
                };
            }
            macro_rules! update_idle_sprite {
                ($sprite:ident) => {
                    // TODO: remove the unnecessary array.
                    self.sprite.index = 0;
                    self.sprite.track = Rc::new([texture_container.$sprite.clone()]);
                };
            }
            match self.state {
                prefabs::PlayerState::IdleDown => {
                    update_idle_sprite!(player_idle_down);
                }
                prefabs::PlayerState::IdleLeft | prefabs::PlayerState::IdleRight => {
                    update_idle_sprite!(player_idle_side);
                }
                prefabs::PlayerState::IdleUp => {
                    update_idle_sprite!(player_idle_up);
                }
                prefabs::PlayerState::MovingDown => {
                    update_moving_sprite!(player_moving_down);
                }
                prefabs::PlayerState::MovingLeft | prefabs::PlayerState::MovingRight => {
                    update_moving_sprite!(player_moving_side);
                }
                prefabs::PlayerState::MovingUp => {
                    update_moving_sprite!(player_moving_up);
                }
            }
        }

        if keyboard_state.is_scancode_pressed(Scancode::Equals)
            && (keyboard_state.is_scancode_pressed(Scancode::LShift)
                || keyboard_state.is_scancode_pressed(Scancode::RShift))
        {
            self.transform.scale *= SCALE_DIFF_FACTOR;
        }
        if keyboard_state.is_scancode_pressed(Scancode::Minus) {
            self.transform.scale *= 1.0 / SCALE_DIFF_FACTOR;
        }
    }
}
