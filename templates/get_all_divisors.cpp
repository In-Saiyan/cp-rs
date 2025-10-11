// Algo: ANAS's custom algo to find all divisors of a number using SPF
#include <bits/stdc++.h>
using namespace std;
#define int long long

const int N=1e6; // Global const for the max number whose prime factorization can be found

vector<int> largest_factor(N+10); // Vector to store largest factor of each number
void spf()
{
    for(int i=2;i<=N;i++)
        if(largest_factor[i]==0)
            for(int j=i;j<=N;j+=i)
                largest_factor[j]=i;
}
// Time Complexity: O(nlog(log(n)))

void getAllDivisors(int i, vector<pair<int,int>> &prime_factors, int cur, vector<int> &div) 
{
    if(cur==(int)(prime_factors.size())) 
    {
        div.push_back(i);
        return;
    }
    int p=prime_factors[cur].first;
    int e=prime_factors[cur].second;
    getAllDivisors(i,prime_factors,cur+1,div);
    int val=i;
    for(int j=1;j<=e;j++)
    {
        val*=p;
        getAllDivisors(val,prime_factors,cur+1,div);
    }
}

vector<int> getAllFactors(int n)
{
    vector<pair<int,int>> prime_factors;
    while(n>1)
    {
        int p=largest_factor[n],c=0;
        while(n%p==0)
        {
            n/=p;
            c++;
        }
        prime_factors.push_back({p,c});
    }
    vector<int> divisors;
    getAllDivisors(1,prime_factors,0,divisors);
    return divisors;
}

int32_t main()
{
    spf();
    // Example
    int n;
    cin >> n;
    vector<int> eg=getAllFactors(n);
    for(int &x:eg)
        cout << x << " ";
    return 0;
}