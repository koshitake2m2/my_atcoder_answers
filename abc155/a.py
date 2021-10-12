a, b, c = map(int, input().split())
if a == b == c:
    print('No')
elif a == b != c:
    print('Yes')
elif a != b == c:
    print('Yes')
elif b != c == a:
    print('Yes')
else:
    print('No')
