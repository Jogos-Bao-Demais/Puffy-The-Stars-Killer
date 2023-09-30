extends CharacterBody2D

const SPEED := 400.0
const ACCELERATION := 1.0
const FRICTION := 1.3

# Get the gravity from the project settings to be synced with RigidBody nodes.
var gravity: float = ProjectSettings.get_setting("physics/2d/default_gravity")

func _physics_process(_delta: float) -> void:
	_move()
	move_and_slide()

func _move() -> void:
	var direction: Vector2 = Vector2(
		Input.get_axis("move_left", "move_right"),
		Input.get_axis("move_up", "move_down")
	)
	
	if direction != Vector2.ZERO:
		velocity.x = lerp(velocity.x, direction.normalized().x * SPEED, ACCELERATION)
		velocity.y = lerp(velocity.y, direction.normalized().y * SPEED, ACCELERATION)
		return
	
	velocity.x = lerp(velocity.x, direction.normalized().x * SPEED, FRICTION)
	velocity.y = lerp(velocity.y, direction.normalized().y * SPEED, FRICTION)
	print(direction)

	
