class Fighter:
    def __init__(self,name,rating = 1500):
        self.name = name
        self.rating = rating
    def update_rating(self,oponent,result):
        k = 50

        if result == 'win':
            expected_score = 1 / (1 + 10 ** ((oponent.rating - self.rating) / 400))
            self.rating += k * (1 - expected_score)

        elif result == 'loss':
            expected_score = 1 / (1 + 10 ** ((self.rating - oponent.rating) / 400))
            self.rating += k * (0 - expected_score)

# Create some fighters
fighter1 = Fighter("Conor McGregor")
fighter2 = Fighter("Khabib Nurmagomedov")

#print initial ratings and name of both fighters
print("Initial Ratings before match")
print(f"Name: {fighter1.name}, Rating: {fighter1.rating}")
print(f"Name: {fighter2.name}, Rating: {fighter2.rating}")

# Simulate a fight and update ratings
fighter1.update_rating(fighter2, "loss")
fighter2.update_rating(fighter1, "win")
print(f"Winner: {fighter2.name}")

# Print ratings and name of both fighters
print("Ratings after match")
print(f"Name: {fighter1.name}, Rating: {fighter1.rating}")
print(f"Name: {fighter2.name}, Rating: {fighter2.rating}")
