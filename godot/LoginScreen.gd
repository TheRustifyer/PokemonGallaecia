extends Control

var RustDLL = preload("res://godot/Background.gdns")
var login_screen = RustDLL.new()

func _ready():
	print('Hi there from main')

func _init():
	var login_screen = RustDLL.new()
	var my_label = login_screen.set_label_text("Label", "Hola")
	print(my_label)
