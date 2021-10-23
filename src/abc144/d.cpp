#include <bits/stdc++.h>
using namespace std;

// 一つWA
void answer1() {
  cin.tie(0);
  ios_base::sync_with_stdio(false);

  int a, b, x;
  cin >> a >> b >> x;
  int max_v = a * a * b;
  double theta;
  if (x == max_v) {
    theta = 0.0;
  } else if (x >= max_v / 2) {
    double bi = (double)(2.0 * (max_v - x)) / (a * a);
    theta = atan(a / bi);
  } else if (x > 0) {
    double ai = (double)(2.0 * x) / (a * b);
    theta = atan(ai / b);
  } else {
    theta = M_PI / 2.0;
  }
  printf("%.10f\n", 90.0 - (theta * 180 / M_PI));
}

void answer2() {
  cin.tie(0);
  ios_base::sync_with_stdio(false);

  int a, b, x;
  cin >> a >> b >> x;
  int max_v = a * a * b;
  double theta;
  if (x >= max_v / 2) {
    double bi = (double)(2.0 * (max_v - x)) / (a * a);
    theta = atan(bi / a);
  } else if (x > 0) {
    double ai = (double)(2.0 * x) / (a * b);
    theta = atan(b / ai);
  }
  printf("%.10f\n", theta * 180 / M_PI);
}

int main() {
  // answer1();
  answer2();
}
