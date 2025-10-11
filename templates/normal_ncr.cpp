// Algo: Normal NCR mod m
#include <bits/stdc++.h>
using namespace std;
#define int long long
const int mod=1e9+7;

// Helper functions
int bin_exp(int a, int b)
{
    int res=1;
    a%mod;
    while(b)
    {
        if(b%2==1) res=(res*a)%mod;
        a=(a*a)%mod;
        b/=2;
    }
    return res;
}

int mod_inverse(int n)
{
    return bin_exp(n,mod-2);
}

int ncr(int n,int r)
{
    if(r<0 || r>n) return 0;
    if(r==n || r==0) return 1;
    int j,ret=1,d;
    r=min(r,n-r);
    for(j=n;j>=n-r+1;j--)
        ret=(ret%mod*j%mod)%mod;
    for(j=1;j<=r;j++)
        d=(d%mod*j%mod)%mod;
    ret=(ret%mod*mod_inverse(d)%mod)%mod;
    return ret;
}
// Time Complexity: O(r)

int32_t main()
{
    // Example
    int n,r;
    cin >> n >> r;
    cout << ncr(n,r) << endl;
}