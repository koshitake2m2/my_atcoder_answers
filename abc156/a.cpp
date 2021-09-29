#include <bits/stdc++.h>
using namespace std;

void solve() {
  cin.tie(0);
  ios_base::sync_with_stdio(false);
  int n, r;
  cin >> n >> r;
  if (n > 10) {
      cout << r << endl;
  } else {
      cout << r + 100 * (10 - n) << endl;
  }
}

int main() {
  solve();
}
