#include <bits/stdc++.h>
using namespace std;
​
#define rep(i, n) for (int i = 0; i < (n); i++)
​
double f(double x, double y) {
    return sqrt(x * x + y * y);
}
​
int main() {
    int N;
    cin >> N;
​
    vector<int> x(N), y(N);
    rep(i, N) cin >> x[i] >> y[i];
​
    vector<int> order(N);
    rep(i, N) order[i] = i;
    // 0, 1, 2, 3, ... N-1
​
    double sum = 0;
    int cnt = 0;
    do {
        for (int i = 0; i + 1 < N; ++i) {
            int k1 = order[i];  // 現在の街の番号
            int k2 = order[i + 1];  // 次訪れる街の番号
            sum += hypot(x[k2] - x[k1], y[k2] - y[k1]);
        }
        ++cnt;
    } while (next_permutation(order.begin(), order.end()));
​
    cout << fixed << setprecision(10); // 小数点以下10桁を常に表示
    cout << sum / cnt << endl;
​
    return 0;
}