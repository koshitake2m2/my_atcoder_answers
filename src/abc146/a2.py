youbis = ['SUN', 'MON', 'TUE', 'WED', 'THU', 'FRI', 'SAT']
today = input()

today_index = youbis.index(today)

next_sunday = 7 - today_index
print(next_sunday)
