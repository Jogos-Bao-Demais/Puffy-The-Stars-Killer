extends CharacterBody2D


@export var health: int = 50
@export var move_speed: float = 250.0
@export var fire_rate: float = 0.2

var bullet = preload("res://Entities/BulletPlaceholder.tscn")

func _process(delta): 
	look_at(get_global_mouse_position())
	
	var bullet_instance = bullet.instantiate()
	bullet_instance.get_

func _physics_process(delta):
	var aread2D = $MeleeRange
	
	velocity.x = move_toward(velocity.x, 0, move_speed)
