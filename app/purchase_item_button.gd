@tool
extends Button

@export var title: String = "<Title>":
  set(newTitle):
    title = newTitle
    if is_inside_tree():
      $Title.text = newTitle

@export var price: float = 0.0:
  set(newPrice):
    price = newPrice
    if is_inside_tree():
      $Price.text = "$%.2f" % newPrice

@export var description: String = "<Description goes here>":
  set(newDesc):
    description = newDesc
    if is_inside_tree():
      $Description.text = newDesc

func _ready():
  $Title.text = title
  $Price.text = "$%.2f" % price
  $Description.text = description
