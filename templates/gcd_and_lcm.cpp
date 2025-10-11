// Algo: GCD and LCM Using Euclidean Algo
#include <bits/stdc++.h>
using namespace std;
#define int long long

int gcd(int a,int b)
{
    while(b)
    {
        a%=b;
        swap(a,b);
    }
    return a;
}
// Time Complexity: O(log(min(a,b)))

int lcm(int a, int b)
{
    return (a*b)/gcd(a,b);
}
// Time Complexity: O(log(min(a,b)))

int32_t main()
{
    // Example;
    int a,b;
    cin >> a >> b;
    cout << gcd(a,b) << " " << lcm(a,b);
    return 0;
}