#include <bits/stdc++.h>
using namespace std;


void solve() {
  cin.tie(0);
  ios_base::sync_with_stdio(false);
  int n, k;
  cin >> n >> k;

  vector<int> keta(40, 0);
  int X = n;
  int sho, amari;
  for (int i = 40; i > 0; i--) {
      sho = X / (int)pow(k, (i-1));
      amari = X % (int)pow(k, (i-1));
      keta[40 - i] = sho;
      X = amari;
  }
//   for (auto a: keta) {
//       cout << a;
//   }
//   cout << endl;

  int count;
  for (int i = 0; i < 40; i++) {
      if (keta[i] != 0) {
          count = i;
          break;
      }
  }
  cout << 40 - count << endl;
}

int main() {
  solve();
}
