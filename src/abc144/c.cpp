#include <bits/stdc++.h>
using namespace std;

void answer1() {
  cin.tie(0);
  ios_base::sync_with_stdio(false);

  long long n;
  cin >> n;
  // 素朴な実装
  long long x = 1, y = 1;
  for (long long i = floor(sqrt(n)); i >= 1; i--) {
    if (n % i == 0) {
      x = i;
      y = n / x;
      break;
    }
  }
  cout << x + y - 2 << endl;
}

int main() { answer1(); }
