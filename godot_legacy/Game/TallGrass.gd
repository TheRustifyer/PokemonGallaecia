extends Node2D

@onready var anim_player = $AnimationPlayer
const grass_overlay_texture = preload("res://gfx/Tilemaps/Grass/stepped_tall_grass.png")
const GrassStepEffect = preload("res://godot/Game/GrassStepEffect.tscn")
var grass_overlay: TextureRect = null


var player_inside: bool = false

# Called when the node enters the scene tree for the first time.
func _ready():
# warning-ignore:return_value_discarded
	get_tree().current_scene.find_child("Player").connect("player_moving", Callable(self, "player_exiting_grass"))
# warning-ignore:return_value_discarded
	#get_tree().current_scene.find_node("Player").connect("player_stopped", self, "player_in_grass")

func player_exiting_grass():
	player_inside = false
	if is_instance_valid(grass_overlay):
		grass_overlay.queue_free()
	
func player_in_grass():
	print("Player in grass")
	if player_inside == true:
		var grass_step_effect = GrassStepEffect.instantiate()
		grass_step_effect.position = position
		get_tree().current_scene.add_child(grass_step_effect)
		
		grass_overlay = TextureRect.new()
		grass_overlay.texture = grass_overlay_texture
		grass_overlay.position = position
		get_tree().current_scene.add_child(grass_overlay)


func _on_Area2D_body_entered(_body):
	player_inside = true
	anim_player.play("Stepped")
	player_in_grass()
