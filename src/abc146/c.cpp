#include <bits/stdc++.h>
using namespace std;

int get_keta(long long n) {
  if (n == 0) {
    return 1;
  }
  int i = 0;
  while (n > 0) {
    n /= 10;
    i++;
  }
  return i;
}

void solve() {
  cin.tie(0);
  ios_base::sync_with_stdio(false);

  long long A, B, X;
  cin >> A >> B >> X;

  // 桁数がdの時の最大のNを求めて、求めたNのうち最大となるものが答えとなる
  long long max_n = 0;
  for (int d = 10; d > 0; d--) {
    long long bunshi = X - B * d;
    if (bunshi < 0) {
      continue;
    }
    long long n = (long long)(bunshi / A);

    // d==10のときはN<10^9なので注意する
    // すなわち、n >= 10^9を満たす時、n = 10^9も満たすを考える。
    if (d == 10) {
      if (n >= pow(10, 9)) {
        n = pow(10, 9);
        max_n = max(n, max_n);
      }
    } else if (get_keta(n) == d) {
      max_n = max(n, max_n);
    }
  }
  cout << max_n << endl;
}

void solve2() {
  cin.tie(0);
  ios_base::sync_with_stdio(false);

  long long A, B, X;
  cin >> A >> B >> X;

  long long low = 0;
  long long high = pow(10, 9) + 1;
  while (high > low + 1) {
    long long mid = (high + low) / 2;
    int d = get_keta(mid);
    if (A * mid + B * d > X) {
      high = mid;
    } else {
      low = mid;
    }
  }
  cout << low << endl;
}

int main() {
  // solve(); // 失敗
  solve2();
}
