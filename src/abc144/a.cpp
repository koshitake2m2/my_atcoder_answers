#include <bits/stdc++.h>
using namespace std;

void answer1() {
  cin.tie(0);
  ios_base::sync_with_stdio(false);

  int a, b;
  cin >> a >> b;
  if (a > 9 || b > 9) {
    cout << -1 << endl;
  } else {
    cout << a * b << endl;
  }
}

int main() { answer1(); }
