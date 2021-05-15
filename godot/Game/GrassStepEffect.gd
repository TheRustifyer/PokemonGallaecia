extends AnimatedSprite

func _ready():
	frame = 0
	playing = true

func _on_GrassStepEffect_animation_finished():
	queue_free()


func _on_grass_step_effect_animation_finished():
	pass # Replace with function body.
