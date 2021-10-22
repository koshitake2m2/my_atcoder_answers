s = input()

day = [
    'SUN',
    'MON',
    'TUE',
    'WED',
    'THU',
    'FRI',
    'SAT',
]

for i in range(len(day)):
    if s == day[i]:
        break

print(7 - i)
