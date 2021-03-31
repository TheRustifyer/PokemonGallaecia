extends Control

var LoginScreen = preload("res://godot/Background.gdns")

func _init():
	print("Constructed!")
	var my_rust_instance = LoginScreen.new()
	var rust_method = my_rust_instance.set_label_text("VBoxContainer/Label", "Text to change")
	print(rust_method) # This method returns null


# Called when the node enters the scene tree for the first time.
func _ready():
	print('Hi there from main!')
	print('***************')
