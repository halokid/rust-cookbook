
class Animal:
  def speak(self):
    print("the " + self.animal_type + " said " + self.noise)

class Dog(Animal):
  def __init__(self):
    self.animal_type = 'dog'
    self.noise = 'woof'



