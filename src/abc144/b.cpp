#include <bits/stdc++.h>
using namespace std;

void answer1() {
  cin.tie(0);
  ios_base::sync_with_stdio(false);

  int n;
  cin >> n;
  bool is_ok = false;
  for (int i = 1; i <= 9; i++) {
    if (n % i == 0) {
      int d = n / i;
      if (1 <= d && d <= 9) {
        is_ok = true;
        break;
      }
    }
  }
  if (is_ok) {
    printf("Yes\n");
  } else {
    printf("No\n");
  }
}

int main() { answer1(); }
