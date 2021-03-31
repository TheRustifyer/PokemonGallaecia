extends Control

var LoginScreen = preload("res://godot/Checker.gdns")

func _init():
	print("Constructed!")
	var my_rust_instance = LoginScreen.new()
	my_rust_instance._ready()


# Called when the node enters the scene tree for the first time.
func _ready():
	print('Hi there from main!')
	print('***************')
