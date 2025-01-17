use bevy::{
      // color::palettes::css::*,
      math::{vec2, vec3},
      prelude::*,
};
use std::time::Duration;
use avian2d::{math::*, parry::shape::SharedShape, prelude::*};

use crate::{resource::Score, states::*};
use crate::componet::*;
use crate::game_time::GameTime;


pub(super) fn plugin(app: &mut App) {

      app.add_observer(on_spawn_player);
      app.add_observer(on_respawn);
    
}

#[derive(Event, Debug)]
pub struct Respawn;

#[derive(Debug, Event)]
pub struct Despawn;

#[derive(Event, Debug)]
pub struct SpawnPlayer(pub Vec3);

fn on_respawn(
      _trigger: Trigger<Respawn>,
      mut cmd: Commands,
) {
      //cmd.trigger(Despawn);
      cmd.trigger(SpawnPlayer(Vec3::new(0.,-74., 10.)));
}

fn on_spawn_player(
      trigger: Trigger<SpawnPlayer>,
      mut cmd: Commands,
      asset_server: Res<AssetServer>,
      mut score: ResMut<Score>,
      mut next_state: ResMut<NextState<GameState>>,
      mut game_time: ResMut<GameTime>,
      mut players:Query<(Entity,&mut Transform, &mut LinearVelocity,&mut AngularVelocity),(With<player>,Without<wall>,Without<brick>)>,
) {
      /* 
      let Ok((ent_player,mut trans, mut linear,mut ang)) = players.get_single_mut() else { return};
      trans.translation = trigger.event().0;
      
      game_time.0 = Duration::ZERO;
      next_state.set(GameState::Wait);
      //println!("{}"u,trigger.event().0);
      */
      **score = 0;
      game_time.0 = Duration::ZERO;
      next_state.set(GameState::Wait);
      if players.is_empty() {
            //player
            cmd.spawn((
                  Sprite {
                  image: asset_server.load("branding/10_64player.png"),
                  ..default()
                  },
                  Transform::from_translation(Vec3::new(0.,-74., 10.)),
                  player,
                  RigidBody::Kinematic,
                  GravityScale(0.0),
                  //Collider::circle(16.),
                  Collider::rectangle(64.,11.), 
                  //SweptCcd::default(),
                  //DebugRender::default().with_collider_color(Color::srgb(1.0, 0.0, 0.0))
                  ))
                  .insert(Name::from("player")
            );
      }
}
