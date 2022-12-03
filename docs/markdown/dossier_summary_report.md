
<style scoped>
table {
    font-size: 10px;
}
.table-wrapper {
    overflow-x: scroll;
}
</style>    
            


## Dossier Assumptions


This table shows all the modeled growth items in a sample `Dossier`. 
The **Index** column details the
location within the dossier:

- `W(w)` Worth item at index `w`. These are the non-financial assets that are part of your
   net worth, such as house, boats, cars, ...

- `A(a)H(h)` Holding with index `h` within account with index `a`. These are financial assets
   within accounts.

- `F(f)` FlowSpec item at index `f`. These are various modeled incomes and expenses (i.e. sequences of cash 
    _flows_) that are typically bound by certain time ranges: _labor income_, _living expense_, _college expense_...

The **Id/Name** column is based on the **label** provided by the user when saving the `dossier`.
The **Category** column specifies the category for this item within the set available
by type (`Worth`, `Holding`, `Instrument`, `Flow`). The **Custom Growth** is an optional
growth the user has associated with the item. These are described in detail below.
The **Value** column indicates some marking of value provided by the user for the item. This
mark may be historic, _hence the date associated with the value_.

### Sample Dossier Financial View

| Index                                                   | Id/Name                                                                       | Category                   | 📁 Custom Growth                   | Value                                                    |
|---------------------------------------------------------|-------------------------------------------------------------------------------|----------------------------|------------------------------------|----------------------------------------------------------|
| <span style="color:green"><em>**Worths**</em></span>    |                                                                               |                            |                                    |                                                          |
| W(0)                                                    | House                                                                         | W(ResidentialRealEstate:1) | <nobr>_</nobr>                     | (2022:＄780,000)                                         |
| W(1)                                                    | Grandpa's Estate                                                              | _                          | <nobr>𝑁(μ=0.1000, σ=0.1000)</nobr> | (2022:＄215,000)                                         |
| <span style="color:green"><em>**Accounts**</em></span>  |                                                                               |                            |                                    |                                                          |
|                                                         | <span style="color:blue"><em>Acc1 - Primary</em></span> (TAXABLE)             |                            |                                    |                                                          |
| A(0)H(0)                                                | SPY                                                                           | H(UsEquityMarket:2)        | <nobr>_</nobr>                     | (1000@(2022:＄376)) Cost(＄332,415)                      |
| A(0)H(1)                                                | VB                                                                            | H(UsSmallEquity:7)         | <nobr>_</nobr>                     | (100@(2022:＄190)) Cost(＄17,523)                        |
| A(0)H(2)                                                | XOM                                                                           | H(UsEquityIndividual:1)    | <nobr>_</nobr>                     | (3200@(2022:＄45)) Cost(＄150,400)                       |
| A(0)H(3)                                                | AAPL                                                                          | I(AAPL:3)                  | <nobr>_</nobr>                     | (300@(2022:＄147)) Cost(＄42,000)                        |
| A(0)H(4)                                                | Bond                                                                          | H(LongTermUsCorpBond:17)   | <nobr>_</nobr>                     | (1@(2022:＄99,857)) Cost(＄98,231)                       |
|                                                         | <span style="color:blue"><em>Acc2 - BrightStart</em></span> (COLLEGE_IRS_529) |                            |                                    |                                                          |
| A(1)H(0)                                                | Blend(2030)/Billy's 529                                                       | _                          | <nobr>𝑁(μ=0.0765, σ=0.1050)</nobr> | (1@(2022:＄13,750)) Cost(＄14,000)                       |
| A(1)H(1)                                                | Blend(2033)/Sara's 529                                                        | H(UsEquityMarket:2)        | <nobr>𝑁(μ=0.0824, σ=0.1130)</nobr> | (1@(2022:＄6,732)) Cost(＄6,000)                         |
|                                                         | <span style="color:blue"><em>Acc3 - 401K</em></span> (TRADITIONAL_IRS_401_K)  |                            |                                    |                                                          |
| A(2)H(0)                                                | VTI                                                                           | H(UsEquityMarket:2)        | <nobr>_</nobr>                     | (166@(2022:＄376)) Cost(＄45,300)                        |
| <span style="color:green"><em>**FlowSpecs**</em></span> |                                                                               |                            |                                    |                                                          |
| F(0)                                                    | Living Expenses Pre-Retirement                                                | F(LivingExpense:10)        | <nobr>_</nobr>                     | _Initial_(2022:＄80,000) _Range_(2022->2036)             |
| F(1)                                                    | Living Expenses Retirement                                                    | F(LivingExpense:10)        | <nobr>_</nobr>                     | _Initial_(2022:＄50,000) _Range_(2036->2066)             |
| F(2)                                                    | College Costs                                                                 | F(CollegeExpense:12)       | <nobr>𝑁(μ=0.0824, σ=0.1130)</nobr> | _Initial_(2022:＄35,000) _Range_(2032->2036)             |
| F(3)                                                    | Custom FlowSpec Example                                                       | _                          | <nobr>_</nobr>                     | _Values_((2028:＄1,500), (2029:＄1,700), (2030:＄1,200)) |
> **Note** Not all items have an associated category. The second worth, `W(1)`
> _Grandpa's Estate_ has no _category_ selected. Maybe there was something special
> about the farm that led the `dossier` creator to believe the system provided categories were
> not a great match. Maybe she knows of some bubbling crude on grandpa's land and
> feels it reasonable to specify a 10% growth instead of the more meager/standard 3% growth.
>
> Similarly, the first holding in the second
> account `A(1)H(0)` called _Blend(2030)/Billy's 529_ represents a holding that the
> user apparently did not find a reasonable category selection. This is not a problem
> because when entering these items the user can specify their own _Normal_ if the
> system categories do not fit well.
> 
> And even if there is a reasonable category selection the user has the option to 
> save their own growth assumption (See `F(2)` _College Costs_).
> These are called **embedded assumptions**. Ideally the majority of all assumptions should come from
> outside the `Dossier` itself so they are more generally applicable and may be applied to runs against 
> any `Dossier`. But sometimes special cases require custom growths.
>
> Similarly, there is no growth associated with `F(3)` labeled _Custom FlowSpec Example_,
> either by nature of a category or with an embedded growth assumption. The reason is because 
> this is a type of `FlowSpec` that does not grow, rather it is a defined set of flows.




Given a `Dossier` we need to decide how those items that can grow, do grow. This process
is called _dossier growth resolution_. We want to be both **flexible** and **easy to use**
and these two characteristics are often at odds. Sometimes the more flexible you make
a system the harder it is to use. To mitigate this, the approach taken is to offer 
considerable (_maybe excessive_) flexibility by allowing all growths and correlations
to be set by the user. However, to ease the burden, no
such assumptions need to be entered as we have system defaults by category. 

There are two methods to associate
growth with any item that grows (e.g. `Worths`, `Holdings`, and `Flows`):

- _The easy way_: Associate the item with a category. Each of the item types have a
  collection of predefined `GrowthAssumptions` to choose from:

    - `Worth` sample categories:
        - `Residential Real Estate`
        - `Commercial Real Estate`
        - ...

    - `Holding` sample categories:
        - `UsEquityMarket`
        - `UsLargeEquity`
        - `UsMidEquity`
        - `UsEquityInstance`
        - ...

    - `Flow` sample categories:
        - `Labor Income`
        - `Living Expense`
        - `College Expense`
        - `Medical Expense`
        - ...

- _The bespoke way_: Enter your own custom `GrowthAssumption`.

> Additionally, these two methods are *not* mutually exclusive. You can associate an annual flow with
> the category of `Living Expense` *and* you can provide your own `GrowthAssumption` for that item.
> With this modeling you can choose _at time of running_ the forecast which is used.

## Resolving Growth Assumptions

The system has _growth assumptions_ associated with categories of `worths`, `holdings` and `flows`.
But in the interest of flexibility, it would be nice if a user could specify **their own** _growth assumptions_
for those categories. For instance, a user with many holdings of type `UsEquityInstance` may not be
happy with the system's growth assumption and want to change that in one place to override all
occurrences. This is possible because users can save their own assumption sets by category. 

### GrowthAssumption

Associating a growth assumption to an item in the dossier has been referenced several times now.
But what is a `GrowthAssumption` in this system? All growth assumptions include two potential
descriptions of how an item may grow:

- `NormalSpec` - With this the user specifies a _mean_ and _standard deviation_, which is the 
  parameterization for a Gaussian (i.e. normal) distribution.

- `RateCurve` - This is simply a _sorted collection of `YearValue` pairs_ that define a curve.

> **Note** These are **not** mutually exclusive.

The primary motivation for specifying the growths as _Normal_ distributions is that it is a 
perfect fit for incorporating the item into a _Monte Carlo_ simulation. Now there is real 
flexibility in modeling. Whenever a growth assumption has been resolved for an item - it
can be included a the simulation using its _normal distribution_, in which case over 
thousand(s) of simulations that distribution will be drawn from. However, there are times
when that gets in the way. Normal distributions over many years are naturally modeling
exponential growth with randomness baked in.
As _standard deviations_ tend to 0 the _Normal_ devolves to 
simply an exponential growth function (i.e. `exp(r^t)`). Sometimes, though, you just want to _shape the rate
curve_ and not assume many consecutive years of **fixed** or **stochastic** normal growth. 
For example, it is not always a great idea
to assume that _Labor Income_ grows at some consistent rate through a person's career. Unfortunately,
the typical growth pattern for labor does not tend to continue increasing until retirement. In fact, more
often the later years of a career show declining values in the workforce. With this in mind
a simple rate curve can support that view:

```rust
[ 
    YearValue{ year: 2022, value: 0.04 }, 
    YearValue{ year: 2030, value: 0.03 }, 
    YearValue{ year: 2040, value: -0.03 }
]
```

In this example an associated item (e.g. "My Programming Job") can be scheduled
to grow at 4% from 2022->2030, at 3% from 2030->2040, and at a decline of 3% from
2040 on.

### Using Other People's Assumptions

Users can save their growth assumptions for categories which are saved
**independent from** any `Dossier`. Additionally, users can reference the growth assumptions
of other users. Finally, the system has its own set of assumptions, _referred to as system
assumptions_ that are modeled under the special designated user called _plusauri_. System
assumptions are always a _backstop_ for resolving a growth assumption. This way there
will always be a sensible growth association.

Two users will be identified by these images in the following tables:



- `plusauri` (💰) - A bag of money image for the system user 
- `test1` (🧪) - A test tube image for the test user




When resolving the growth assumptions you **may** specify an _ordered list_ of users whose assumptions
you'd like to use in addition to your own. If the list is empty it resorts to the system values.

> **_The following numbers are (randomly generated) test data... please do not use._**

Here are sample test system growth assumptions for a smallish set of categories for the system
user `plusauri` 💰

#### System Assumptions (Test Data)


| UserId | OutlookId | ItemId                     | NormalSpec            |
|--------|-----------|----------------------------|-----------------------|
| 💰     | 👉        | F(CollegeExpense:12)       | 𝑁(μ=0.1063, σ=0.1911) |
| 💰     | 👉        | F(LivingExpense:10)        | 𝑁(μ=0.0259, σ=0.0078) |
| 💰     | 👉        | F(SocialSecurityIncome:4)  | 𝑁(μ=0.1185, σ=0.2306) |
| 💰     | 👉        | H(Commodities:23)          | 𝑁(μ=0.1010, σ=0.1757) |
| 💰     | 👉        | H(EmergingEquityMarket:12) | 𝑁(μ=0.0991, σ=0.1812) |
| 💰     | 👉        | H(LongTermUsCorpBond:17)   | 𝑁(μ=0.1015, σ=0.1782) |
| 💰     | 👉        | H(LongTermUsGovBond:16)    | 𝑁(μ=0.1144, σ=0.1668) |
| 💰     | 👉        | H(Tips:13)                 | 𝑁(μ=0.0841, σ=0.1855) |
| 💰     | 👉        | H(UsEquityIndividual:1)    | 𝑁(μ=0.1283, σ=0.1433) |
| 💰     | 👉        | H(UsEquityMarket:2)        | 𝑁(μ=0.0912, σ=0.1510) |
| 💰     | 👉        | H(UsGov10YrBond:14)        | 𝑁(μ=0.0800, σ=0.1966) |
| 💰     | 👉        | H(UsLargeEquityMarket:4)   | 𝑁(μ=0.1017, σ=0.2192) |
| 💰     | 👉        | H(UsMidEquityMarket:6)     | 𝑁(μ=0.1224, σ=0.1502) |
| 💰     | 👉        | H(UsSmallEquity:7)         | 𝑁(μ=0.1198, σ=0.1952) |
| 💰     | 👉        | I(AAPL:3)                  | 𝑁(μ=0.1696, σ=0.2152) |
| 💰     | 👉        | I(MU:322)                  | 𝑁(μ=0.1143, σ=0.1837) |
| 💰     | 👉        | W(ResidentialRealEstate:1) | 𝑁(μ=0.0553, σ=0.1174) |


And here are the test assumptions for the `test1` 🧪 user that she's chosen to save.

> Again this is an example of what is supported, not what is required or
> even encouraged for the basic user. However, think of a planner with 
> many clients who has his own views
> and wants to save them for reuse. Also, note for simplicity the following
> discussion is only focused on specifying the _Normal_ characteristics of
> items. Saving rate curves is also supported just not the focus here.

#### Test Assumptions



| UserId | OutlookId | ItemId                     | NormalSpec            |
|--------|-----------|----------------------------|-----------------------|
| 🧪     | 👉        | F(CollegeExpense:12)       | 𝑁(μ=0.0971, σ=0.2207) |
| 🧪     | 👉        | F(LivingExpense:10)        | 𝑁(μ=0.0372, σ=0.0081) |
| 🧪     | 👉        | F(SocialSecurityIncome:4)  | 𝑁(μ=0.1050, σ=0.1771) |
| 🧪     | 👉        | H(LongTermUsGovBond:16)    | 𝑁(μ=0.0958, σ=0.2116) |
| 🧪     | 👉        | H(UsEquityMarket:2)        | 𝑁(μ=0.0855, σ=0.1772) |
| 🧪     | 👉        | H(UsGov10YrBond:14)        | 𝑁(μ=0.0974, σ=0.1939) |
| 🧪     | 👉        | I(AAPL:3)                  | 𝑁(μ=0.1055, σ=0.2074) |
| 🧪     | 👉        | I(MU:322)                  | 𝑁(μ=0.1177, σ=0.2232) |
| 🧪     | 👉        | W(ResidentialRealEstate:1) | 𝑁(μ=0.0559, σ=0.1117) |




### Sources For Growth Assumptions

When resolving growth assumptions the sources are:

- One or more sets of saved assumptions per user, _optionally broken out by a concept
  called **outlook** - see below_.

- Growth assumptions embedded directly in the dossier 📁

You control the resolution process by parameterizing the forecast request with:

- An ordered list of users providing search preference. If you are fond of planner
  _Rick_'s assumptions then your list simply includes that one id `[Rick]`.
  Of course resolving from multiple users is supported `[Rick, Suze, ...]` with the search 
  looking for assumptions within each in turn. In all cases it
  is assumed that the system `plusauri` assumptions are a backstop, so the
  first is actually `[Rick, plusauri]` and the second actually `[Rick, Suze, ..., plusauri]`.
  The list may also be empty `[]`, which is effectively `[plusauri]`.

- In addition to users there is one more dimension of variance called _outlook_.
  These are the predefined outlooks:
  - _standard_ - 👉
  - _gloomy_ - 👎
  - _bright_ - 👍
  - _hold_on_tight_

  The default is _standard_ 👉 and only the really determined might find need for fleshing
  out assumptions for each. By default all forecasts look for assumptions in the
  _standard_ outlook as a backstop.

- Specifying a preference for _user assumptions_ or for 
  _embedded growth assumptions_ (i.e. from the dossier 📁). In either case an absence of the preferred result will
  still resort to the other type. When resolving growth assumptions for a forecast 
  you can specify the `ResolutionMode`:

  - _Prefer User Assumptions_
  - _Prefer Dossier_
  
  If both are available this mode flag tells which to use. If only one is available, that is used.
  
Putting these together, then, each forecast is parameterized by a simple set
of attributes:




```rust
pub struct ResolutionSpec<'a> {
    /// The users ordered by preference for lookup
    pub ordered_users: &'a [u32],
    /// The outlook for the search (e.g. `standard`, `gloomy`...).
    pub outlook_id: u32,
    /// Growth assumptions provided by users/outlook.
    pub user_market_assumptions: &'a UserMarketAssumptions,
    /// How to resolve in case of multiple growths per item
    pub resolution_mode: ResolutionMode,
}
```

### Example Growth Resolutions

Now, lets see how growth is resolved by using the search on the single system user 
`[plusauri]`, using the _standard_ outlook and only varying the `ResolutionMode`.

#### Searching With System User Assumptions

- First, when specifying _PreferUserAssumptions_, the following is resolved:



| Index                                                            | User | GrowthItem                 | GrowthAssumption                      |
|------------------------------------------------------------------|------|----------------------------|---------------------------------------|
| <nobr>W(0):House-(2022:＄780,000)</nobr>                         | 💰   | W(ResidentialRealEstate:1) | <nobr>⚙️ 𝑁(μ=0.0553, σ=0.1174)</nobr>  |
| <nobr>W(1):Grandpa's Estate-(2022:＄215,000)</nobr>              | _    | _                          | <nobr>📁 𝑁(μ=0.1000, σ=0.1000)</nobr> |
| <nobr>A(0)H(0):Acc1 - Primary:SPY:</nobr>                        | 💰   | H(UsEquityMarket:2)        | <nobr>⚙️ 𝑁(μ=0.0912, σ=0.1510)</nobr>  |
| <nobr>A(0)H(1):Acc1 - Primary:VB:</nobr>                         | 💰   | H(UsSmallEquity:7)         | <nobr>⚙️ 𝑁(μ=0.1198, σ=0.1952)</nobr>  |
| <nobr>A(0)H(2):Acc1 - Primary:XOM:</nobr>                        | 💰   | H(UsEquityIndividual:1)    | <nobr>⚙️ 𝑁(μ=0.1283, σ=0.1433)</nobr>  |
| <nobr>A(0)H(3):Acc1 - Primary:AAPL:</nobr>                       | 💰   | I(AAPL:3)                  | <nobr>⚙️ 𝑁(μ=0.1696, σ=0.2152)</nobr>  |
| <nobr>A(0)H(4):Acc1 - Primary:Bond:</nobr>                       | 💰   | H(LongTermUsCorpBond:17)   | <nobr>⚙️ 𝑁(μ=0.1015, σ=0.1782)</nobr>  |
| <nobr>A(1)H(0):Acc2 - BrightStart:Blend(2030):Billy's 529</nobr> | _    | _                          | <nobr>📁 𝑁(μ=0.0765, σ=0.1050)</nobr> |
| <nobr>A(1)H(1):Acc2 - BrightStart:Blend(2033):Sara's 529</nobr>  | 💰   | H(UsEquityMarket:2)        | <nobr>⚙️ 𝑁(μ=0.0912, σ=0.1510)</nobr>  |
| <nobr>A(2)H(0):Acc3 - 401K:VTI:</nobr>                           | 💰   | H(UsEquityMarket:2)        | <nobr>⚙️ 𝑁(μ=0.0912, σ=0.1510)</nobr>  |
| <nobr>F(0):Living Expenses Pre-Retirement</nobr>                 | 💰   | F(LivingExpense:10)        | <nobr>⚙️ 𝑁(μ=0.0259, σ=0.0078)</nobr>  |
| <nobr>F(1):Living Expenses Retirement</nobr>                     | 💰   | F(LivingExpense:10)        | <nobr>⚙️ 𝑁(μ=0.0259, σ=0.0078)</nobr>  |
| <nobr>F(2):College Costs</nobr>                                  | 💰   | F(CollegeExpense:12)       | <nobr>⚙️ 𝑁(μ=0.1063, σ=0.1911)</nobr>  |
| <nobr>F(3):Custom FlowSpec Example</nobr>                        | _    | _                          | _                                     |



- Second, when specifying _PreferDossier_, the following is resolved:



| Index                                                            | User | GrowthItem                 | GrowthAssumption                      |
|------------------------------------------------------------------|------|----------------------------|---------------------------------------|
| <nobr>W(0):House-(2022:＄780,000)</nobr>                         | 💰   | W(ResidentialRealEstate:1) | <nobr>⚙️ 𝑁(μ=0.0553, σ=0.1174)</nobr>  |
| <nobr>W(1):Grandpa's Estate-(2022:＄215,000)</nobr>              | _    | _                          | <nobr>📁 𝑁(μ=0.1000, σ=0.1000)</nobr> |
| <nobr>A(0)H(0):Acc1 - Primary:SPY:</nobr>                        | 💰   | H(UsEquityMarket:2)        | <nobr>⚙️ 𝑁(μ=0.0912, σ=0.1510)</nobr>  |
| <nobr>A(0)H(1):Acc1 - Primary:VB:</nobr>                         | 💰   | H(UsSmallEquity:7)         | <nobr>⚙️ 𝑁(μ=0.1198, σ=0.1952)</nobr>  |
| <nobr>A(0)H(2):Acc1 - Primary:XOM:</nobr>                        | 💰   | H(UsEquityIndividual:1)    | <nobr>⚙️ 𝑁(μ=0.1283, σ=0.1433)</nobr>  |
| <nobr>A(0)H(3):Acc1 - Primary:AAPL:</nobr>                       | 💰   | I(AAPL:3)                  | <nobr>⚙️ 𝑁(μ=0.1696, σ=0.2152)</nobr>  |
| <nobr>A(0)H(4):Acc1 - Primary:Bond:</nobr>                       | 💰   | H(LongTermUsCorpBond:17)   | <nobr>⚙️ 𝑁(μ=0.1015, σ=0.1782)</nobr>  |
| <nobr>A(1)H(0):Acc2 - BrightStart:Blend(2030):Billy's 529</nobr> | _    | _                          | <nobr>📁 𝑁(μ=0.0765, σ=0.1050)</nobr> |
| <nobr>A(1)H(1):Acc2 - BrightStart:Blend(2033):Sara's 529</nobr>  | _    | H(UsEquityMarket:2)        | <nobr>📁 𝑁(μ=0.0824, σ=0.1130)</nobr> |
| <nobr>A(2)H(0):Acc3 - 401K:VTI:</nobr>                           | 💰   | H(UsEquityMarket:2)        | <nobr>⚙️ 𝑁(μ=0.0912, σ=0.1510)</nobr>  |
| <nobr>F(0):Living Expenses Pre-Retirement</nobr>                 | 💰   | F(LivingExpense:10)        | <nobr>⚙️ 𝑁(μ=0.0259, σ=0.0078)</nobr>  |
| <nobr>F(1):Living Expenses Retirement</nobr>                     | 💰   | F(LivingExpense:10)        | <nobr>⚙️ 𝑁(μ=0.0259, σ=0.0078)</nobr>  |
| <nobr>F(2):College Costs</nobr>                                  | _    | F(CollegeExpense:12)       | <nobr>📁 𝑁(μ=0.0824, σ=0.1130)</nobr> |
| <nobr>F(3):Custom FlowSpec Example</nobr>                        | _    | _                          | _                                     |


> **Note** In all cases the resolved user is `plusauri` since the _ordered_users_ list
> only included that user. In the first example the icon of all **GrowthAssumption** entries, except one,
> is decorated with `⚙️` indicating they come from system (or user) assumptions. The holding
> _A(1)H(0):Acc2 - BrightStart:Blend(2030):Billy's 529_ is the exception, decorated with a folder
> icon `📁` indicating the assumption was an embedded assumption from the `Dossier`. This was
> the resulting resolution because no category was associated with _Billy's 529_, instead a
> custom growth assumption was associated with it.
>
> In the second example, three more entries:
>
> - _W(1):Grandpa's Estate-(2022:＄215,000)_,
> - _A(1)H(1):Acc2 - BrightStart:Blend(2033):Sara's 529_, and 
> - _F(2):College Costs_
>
> resolved to pulling the growth from the `Dossier` because
> the flag specified that preference.

#### Searching With `Test1` User Assumptions

With the previous examples, all the assumptions, except the _embedded growth assumptions_ come
from the system. But maybe a user such as _test1_ knows more about financial planning.
Or maybe she wants to capture a more representative set of statistics than those of the
system. To that end she has created her own set of assumptions as shown above in 
[Test User Assumptions](#test-assumptions). Running with her own id in the _ordered_users_
list (i.e. `[test1]`) gives the following results for each case:

- First, when specifying _PreferUserAssumptions_, the following is resolved:



| Index                                                            | User | GrowthItem                 | GrowthAssumption                      |
|------------------------------------------------------------------|------|----------------------------|---------------------------------------|
| <nobr>W(0):House-(2022:＄780,000)</nobr>                         | 🧪   | W(ResidentialRealEstate:1) | <nobr>⚙️ 𝑁(μ=0.0559, σ=0.1117)</nobr>  |
| <nobr>W(1):Grandpa's Estate-(2022:＄215,000)</nobr>              | _    | _                          | <nobr>📁 𝑁(μ=0.1000, σ=0.1000)</nobr> |
| <nobr>A(0)H(0):Acc1 - Primary:SPY:</nobr>                        | 🧪   | H(UsEquityMarket:2)        | <nobr>⚙️ 𝑁(μ=0.0855, σ=0.1772)</nobr>  |
| <nobr>A(0)H(1):Acc1 - Primary:VB:</nobr>                         | 💰   | H(UsSmallEquity:7)         | <nobr>⚙️ 𝑁(μ=0.1198, σ=0.1952)</nobr>  |
| <nobr>A(0)H(2):Acc1 - Primary:XOM:</nobr>                        | 💰   | H(UsEquityIndividual:1)    | <nobr>⚙️ 𝑁(μ=0.1283, σ=0.1433)</nobr>  |
| <nobr>A(0)H(3):Acc1 - Primary:AAPL:</nobr>                       | 🧪   | I(AAPL:3)                  | <nobr>⚙️ 𝑁(μ=0.1055, σ=0.2074)</nobr>  |
| <nobr>A(0)H(4):Acc1 - Primary:Bond:</nobr>                       | 💰   | H(LongTermUsCorpBond:17)   | <nobr>⚙️ 𝑁(μ=0.1015, σ=0.1782)</nobr>  |
| <nobr>A(1)H(0):Acc2 - BrightStart:Blend(2030):Billy's 529</nobr> | _    | _                          | <nobr>📁 𝑁(μ=0.0765, σ=0.1050)</nobr> |
| <nobr>A(1)H(1):Acc2 - BrightStart:Blend(2033):Sara's 529</nobr>  | 🧪   | H(UsEquityMarket:2)        | <nobr>⚙️ 𝑁(μ=0.0855, σ=0.1772)</nobr>  |
| <nobr>A(2)H(0):Acc3 - 401K:VTI:</nobr>                           | 🧪   | H(UsEquityMarket:2)        | <nobr>⚙️ 𝑁(μ=0.0855, σ=0.1772)</nobr>  |
| <nobr>F(0):Living Expenses Pre-Retirement</nobr>                 | 🧪   | F(LivingExpense:10)        | <nobr>⚙️ 𝑁(μ=0.0372, σ=0.0081)</nobr>  |
| <nobr>F(1):Living Expenses Retirement</nobr>                     | 🧪   | F(LivingExpense:10)        | <nobr>⚙️ 𝑁(μ=0.0372, σ=0.0081)</nobr>  |
| <nobr>F(2):College Costs</nobr>                                  | 🧪   | F(CollegeExpense:12)       | <nobr>⚙️ 𝑁(μ=0.0971, σ=0.2207)</nobr>  |
| <nobr>F(3):Custom FlowSpec Example</nobr>                        | _    | _                          | _                                     |



- Second, when specifying _PreferDossier_, the following is resolved:



| Index                                                            | User | GrowthItem                 | GrowthAssumption                      |
|------------------------------------------------------------------|------|----------------------------|---------------------------------------|
| <nobr>W(0):House-(2022:＄780,000)</nobr>                         | 🧪   | W(ResidentialRealEstate:1) | <nobr>⚙️ 𝑁(μ=0.0559, σ=0.1117)</nobr>  |
| <nobr>W(1):Grandpa's Estate-(2022:＄215,000)</nobr>              | _    | _                          | <nobr>📁 𝑁(μ=0.1000, σ=0.1000)</nobr> |
| <nobr>A(0)H(0):Acc1 - Primary:SPY:</nobr>                        | 🧪   | H(UsEquityMarket:2)        | <nobr>⚙️ 𝑁(μ=0.0855, σ=0.1772)</nobr>  |
| <nobr>A(0)H(1):Acc1 - Primary:VB:</nobr>                         | 💰   | H(UsSmallEquity:7)         | <nobr>⚙️ 𝑁(μ=0.1198, σ=0.1952)</nobr>  |
| <nobr>A(0)H(2):Acc1 - Primary:XOM:</nobr>                        | 💰   | H(UsEquityIndividual:1)    | <nobr>⚙️ 𝑁(μ=0.1283, σ=0.1433)</nobr>  |
| <nobr>A(0)H(3):Acc1 - Primary:AAPL:</nobr>                       | 🧪   | I(AAPL:3)                  | <nobr>⚙️ 𝑁(μ=0.1055, σ=0.2074)</nobr>  |
| <nobr>A(0)H(4):Acc1 - Primary:Bond:</nobr>                       | 💰   | H(LongTermUsCorpBond:17)   | <nobr>⚙️ 𝑁(μ=0.1015, σ=0.1782)</nobr>  |
| <nobr>A(1)H(0):Acc2 - BrightStart:Blend(2030):Billy's 529</nobr> | _    | _                          | <nobr>📁 𝑁(μ=0.0765, σ=0.1050)</nobr> |
| <nobr>A(1)H(1):Acc2 - BrightStart:Blend(2033):Sara's 529</nobr>  | _    | H(UsEquityMarket:2)        | <nobr>📁 𝑁(μ=0.0824, σ=0.1130)</nobr> |
| <nobr>A(2)H(0):Acc3 - 401K:VTI:</nobr>                           | 🧪   | H(UsEquityMarket:2)        | <nobr>⚙️ 𝑁(μ=0.0855, σ=0.1772)</nobr>  |
| <nobr>F(0):Living Expenses Pre-Retirement</nobr>                 | 🧪   | F(LivingExpense:10)        | <nobr>⚙️ 𝑁(μ=0.0372, σ=0.0081)</nobr>  |
| <nobr>F(1):Living Expenses Retirement</nobr>                     | 🧪   | F(LivingExpense:10)        | <nobr>⚙️ 𝑁(μ=0.0372, σ=0.0081)</nobr>  |
| <nobr>F(2):College Costs</nobr>                                  | _    | F(CollegeExpense:12)       | <nobr>📁 𝑁(μ=0.0824, σ=0.1130)</nobr> |
| <nobr>F(3):Custom FlowSpec Example</nobr>                        | _    | _                          | _                                     |


> **Note** In these runs the user `test1`'s growth assumptions, where present,
> have take precedence (e.g. `ResidentialRealEstate` and `UsEquityMarket`.).
> For those cases where `test1` has entered no assumptions the `plusauri`
> is the backstop.

## Resolving Correlations

### User Category Correlations

To run a _Monte Carlo_ simulation you need growths of all items participating in
the forecast in a stochastic way. We are using using a
_normal distributions_ for the financial holding returns as well as growths associated
with `worths` and `flows`. That is a good start; but to better understand the distribution
of forecasts we need a way to incorporate
those correlations into the forecast. Resolving growth in a flexible way was a bit involved
and the situation is similar for correlations. This section will go over the correlation resolution 
process in detail.

Once again the primary concern is for both ease of use and flexibility. In an approach similar 
to growth assumptions, users can use the system correlations or provide their own. As a refresher,
correlations are simply normalized values between -1 and 1 indicating the extent to which two
variables tend to move together. A correlation of 1 is _perfect correlation_ and the values will
move together in perfect step. Likewise, a correlation of -1 is _perfect negative correlation_
and the values will move in opposite directions in perfect step.

Here again are correlations for the system user, `plusauri`.

> *Note* This is randomly generated test data - not intended for use.



| User | Outlook | Row                        | Column                   | Correlation |
|------|---------|----------------------------|--------------------------|-------------|
| 💰   | 👉      | H(UsEquityIndividual:1)    | H(UsEquityMarket:2)      | 0.6324      |
| 💰   | 👉      | H(UsEquityMarket:2)        | F(CollegeExpense:12)     | 0.0992      |
| 💰   | 👉      | H(UsEquityMarket:2)        | F(LivingExpense:10)      | 0.0792      |
| 💰   | 👉      | H(UsEquityMarket:2)        | H(Commodities:23)        | 0.1737      |
| 💰   | 👉      | H(UsEquityMarket:2)        | H(LongTermUsCorpBond:17) | 0.2098      |
| 💰   | 👉      | H(UsEquityMarket:2)        | H(UsLargeEquity:3)       | 0.9389      |
| 💰   | 👉      | H(UsEquityMarket:2)        | H(UsSmallEquity:7)       | 0.6955      |
| 💰   | 👉      | H(UsLargeEquity:3)         | H(LongTermUsGovBond:16)  | 0.1847      |
| 💰   | 👉      | H(UsLargeEquity:3)         | H(UsMidEquity:5)         | 0.6412      |
| 💰   | 👉      | H(UsLargeEquity:3)         | H(UsSmallEquity:7)       | 0.6373      |
| 💰   | 👉      | H(UsSmallEquity:7)         | F(CollegeExpense:12)     | 0.1423      |
| 💰   | 👉      | H(UsSmallEquity:7)         | F(LivingExpense:10)      | 0.1499      |
| 💰   | 👉      | I(AAPL:3)                  | H(UsEquityMarket:2)      | 0.3238      |
| 💰   | 👉      | W(ResidentialRealEstate:1) | H(UsLargeEquity:3)       | 0.5072      |
| 💰   | 👉      | W(ResidentialRealEstate:1) | H(UsSmallEquity:7)       | 0.2976      |



That table is showing the correlations, or interdependence, of pair-wise combinations
of categories. It is important to understand that there is no requirement that all pair-wise
combinations have a correlation. Any pairwise correlation that has no `CorrelationEntry`, but whose
two constituents still have relations to other categories, will be assumed to have a
0 correlation.

This is another table showing the same dataset as a 2D matrix that those familiar with linear
algebra may recognize.

#### Plusauri User Modeled Correlations

<div class="table-wrapper" markdown="block">



|                            | I(AAPL:3)      | H(UsEquityIndividual:1) | H(UsEquityMarket:2) | H(UsLargeEquity:3) | H(UsMidEquity:5) | H(UsSmallEquity:7) | H(LongTermUsGovBond:16) | H(LongTermUsCorpBond:17) | H(Commodities:23) | W(ResidentialRealEstate:1) | F(LivingExpense:10) | F(CollegeExpense:12) |
|----------------------------|----------------|-------------------------|---------------------|--------------------|------------------|--------------------|-------------------------|--------------------------|-------------------|----------------------------|---------------------|----------------------|
| I(AAPL:3)                  | _              | _                       | _                   | _                  | _                | _                  | _                       | _                        | _                 | _                          | _                   | _                    |
| H(UsEquityIndividual:1)    | _              | _                       | _                   | _                  | _                | _                  | _                       | _                        | _                 | _                          | _                   | _                    |
| H(UsEquityMarket:2)        | 💰👉 ρ(0.3238) | 💰👉 ρ(0.6324)          | _                   | _                  | _                | _                  | _                       | _                        | _                 | _                          | _                   | _                    |
| H(UsLargeEquity:3)         | _              | _                       | 💰👉 ρ(0.9389)      | _                  | _                | _                  | _                       | _                        | _                 | _                          | _                   | _                    |
| H(UsMidEquity:5)           | _              | _                       | _                   | 💰👉 ρ(0.6412)     | _                | _                  | _                       | _                        | _                 | _                          | _                   | _                    |
| H(UsSmallEquity:7)         | _              | _                       | 💰👉 ρ(0.6955)      | 💰👉 ρ(0.6373)     | _                | _                  | _                       | _                        | _                 | _                          | _                   | _                    |
| H(LongTermUsGovBond:16)    | _              | _                       | _                   | 💰👉 ρ(0.1847)     | _                | _                  | _                       | _                        | _                 | _                          | _                   | _                    |
| H(LongTermUsCorpBond:17)   | _              | _                       | 💰👉 ρ(0.2098)      | _                  | _                | _                  | _                       | _                        | _                 | _                          | _                   | _                    |
| H(Commodities:23)          | _              | _                       | 💰👉 ρ(0.1737)      | _                  | _                | _                  | _                       | _                        | _                 | _                          | _                   | _                    |
| W(ResidentialRealEstate:1) | _              | _                       | _                   | 💰👉 ρ(0.5072)     | _                | 💰👉 ρ(0.2976)     | _                       | _                        | _                 | _                          | _                   | _                    |
| F(LivingExpense:10)        | _              | _                       | 💰👉 ρ(0.0792)      | _                  | _                | 💰👉 ρ(0.1499)     | _                       | _                        | _                 | _                          | _                   | _                    |
| F(CollegeExpense:12)       | _              | _                       | 💰👉 ρ(0.0992)      | _                  | _                | 💰👉 ρ(0.1423)     | _                       | _                        | _                 | _                          | _                   | _                    |


</div>
<h6 style="text-align: center;">Scrollable View of Dense Correlation Matrix</h6>   

> **Note** The same data is presented but in a much more area intensive manner.
> But the benefit of this view is it can be simpler to find a correlation. Observe
> that many of the pair-wise slots are empty. That is fine, those will have 0 
> correlations. Once again, this is just test data presented to illustrate
> how resolving correlations work. 

As with growth assumptions the users can save their own correlations. Here is the
condensed view of user `test1`'s modeled correlations:


#### Test1 User Modeled Correlations

<div class="table-wrapper" markdown="block">



| User | Outlook | Row                        | Column                   | Correlation |
|------|---------|----------------------------|--------------------------|-------------|
| 🧪   | 👉      | H(UsEquityIndividual:1)    | H(UsEquityMarket:2)      | 0.7694      |
| 🧪   | 👉      | H(UsEquityMarket:2)        | H(Commodities:23)        | 0.0251      |
| 🧪   | 👉      | H(UsLargeEquity:3)         | H(LongTermUsCorpBond:17) | 0.2791      |
| 🧪   | 👉      | H(UsLargeEquity:3)         | H(UsMidEquity:5)         | 0.7408      |
| 🧪   | 👉      | I(AAPL:3)                  | H(UsEquityMarket:2)      | 0.2594      |
| 🧪   | 👉      | W(ResidentialRealEstate:1) | H(UsLargeEquity:3)       | 0.1750      |


</div>
<h6 style="text-align: center;">Scrollable View of Dense Correlation Matrix</h6>   

Now as you might expect the process of picking a correlation is simply choosing
the most preferred correlation among those available. Here is a _merged correlation_
matrix with an _ordered users_ set to `[test1, plusauri]`.

> **Note** An important point about merging correlations is that only picking a few
> correlations from one place and merging with those from another, as 
> opposed to a complete set of measured correlations can introduce
> error. It is not too difficult to select a set of pair-wise correlations that are
> technically not well-formed. For example, you could give a very high correlation
> between IBM and MSFT. Similarly you could give a very high negative correlation
> between MSFT and CSCO. Intuitively this says IBM and MSFT move up or down together
> and MSFT and CSCO move in opposite directions. What if you then said IBM and CSCO
> are perfectly correlated or perfectly negatively correlated? You are saying something
> that does not really make sense and that causes issues with generating the correlated
> returns.

The two previous correlations [Plusauri User Correlations](#plusauri-user-modeled-correlations)
and [Test1 User Correlations](#test1-user-modeled-correlations) both represent the set of
correlations those users have modeled. In the case of `plusauri` that will be much larger,
as complete as possible and with real data. The entries for the `test1` user are big or small
to whatever extent they want to use their own correlations. But these are _totally independent_ from any
`dossiers` which provides leverage. In fact user `test1` may save correlations for categories
not used within this particular `dossier` but that may be used in others.
This approach allows them to be used to forecast any or many `dossiers`.

So how does a forecast on a `dossier` resolve correlations from these _user modeled correlations_?
The resolution process, using the _ordered users_ first merges all the user assumptions and filters
those to only categories that are referenced by the `dossier` being forecast.

#### Merged User Correlation Sparse View


| Row Index                  | Column Index            | Correlation    |
|----------------------------|-------------------------|----------------|
| H(UsEquityMarket:2)        | I(AAPL:3)               | 🧪👉 ρ(0.2594) |
| H(UsEquityMarket:2)        | H(UsEquityIndividual:1) | 🧪👉 ρ(0.7694) |
| H(UsSmallEquity:7)         | H(UsEquityMarket:2)     | 💰👉 ρ(0.6955) |
| H(LongTermUsCorpBond:17)   | H(UsEquityMarket:2)     | 💰👉 ρ(0.2098) |
| W(ResidentialRealEstate:1) | H(UsSmallEquity:7)      | 💰👉 ρ(0.2976) |
| F(LivingExpense:10)        | H(UsEquityMarket:2)     | 💰👉 ρ(0.0792) |
| F(LivingExpense:10)        | H(UsSmallEquity:7)      | 💰👉 ρ(0.1499) |
| F(CollegeExpense:12)       | H(UsEquityMarket:2)     | 💰👉 ρ(0.0992) |
| F(CollegeExpense:12)       | H(UsSmallEquity:7)      | 💰👉 ρ(0.1423) |



It then constructs a correlation matrix based on all `system ids` being
referenced in the `dossier`. Note in the two user modeled assumptions sets there is a `H(Commodities:23)`
entry but the sample `dossier` has no reference to commodities. Hence in this view that correlation does
not enter play. In this case `test1` user has only modeled pairs that affected correlations in the resulting
set.

#### Merged User Correlation Matrix View

Here is the same dataset viewed in matrix form.



|                            | I(AAPL:3)      | H(UsEquityIndividual:1) | H(UsEquityMarket:2) | H(UsSmallEquity:7) | H(LongTermUsCorpBond:17) | W(ResidentialRealEstate:1) | F(LivingExpense:10) | F(CollegeExpense:12) |
|----------------------------|----------------|-------------------------|---------------------|--------------------|--------------------------|----------------------------|---------------------|----------------------|
| I(AAPL:3)                  | 1.0            | _                       | _                   | _                  | _                        | _                          | _                   | _                    |
| H(UsEquityIndividual:1)    | _              | 1.0                     | _                   | _                  | _                        | _                          | _                   | _                    |
| H(UsEquityMarket:2)        | 🧪👉 ρ(0.2594) | 🧪👉 ρ(0.7694)          | 1.0                 | _                  | _                        | _                          | _                   | _                    |
| H(UsSmallEquity:7)         | _              | _                       | 💰👉 ρ(0.6955)      | 1.0                | _                        | _                          | _                   | _                    |
| H(LongTermUsCorpBond:17)   | _              | _                       | 💰👉 ρ(0.2098)      | _                  | 1.0                      | _                          | _                   | _                    |
| W(ResidentialRealEstate:1) | _              | _                       | _                   | 💰👉 ρ(0.2976)     | _                        | 1.0                        | _                   | _                    |
| F(LivingExpense:10)        | _              | _                       | 💰👉 ρ(0.0792)      | 💰👉 ρ(0.1499)     | _                        | _                          | 1.0                 | _                    |
| F(CollegeExpense:12)       | _              | _                       | 💰👉 ρ(0.0992)      | 💰👉 ρ(0.1423)     | _                        | _                          | _                   | 1.0                  |



This matrix looks quite similar to the [Plusauri User Modeled Correlations](#plusauri-user-modeled-correlations)
with the exception that those two `test1` modeled correlations have made it into the matrix. Nice - Score
for flexibility!

### Dossier Correlations

Now we've seen how correlations between items associated with categories are resolved.
But what about items that are not well-described by system categories for which the
user has entered custom growth assumptions? Well, if the system has no growth assumption
applicable it will certainly have no correlations to any of its other categories. 
But then, no correlations simply means a correlation of 0 which, which may be good enough. If, however, 
you have your own correlations for those uncategorized items you can save those,
_embedded in the `dossier`_ just like the embedded _growth assumptions_. The indexing for these
correlations will be different since they are not based on the system categories. Rather,
they are tied directly to the items in the dossier and they use those indices seen 
in the first [Sample Dossier Financial View](#sample-dossier-financial-view).

For simplicity we'll save one correlation in the `dossier` to show how this works.
Suppose `test1` user, who knows of some black gold/texas tea on _Grandpa's Estate_
and she suspects that will be fairly highly correlated with her _Exxon Mobil_ (`XOM`).
For this pair she saves a correlation of 0.75 in the dossier.  



|          | W(1)         | A(0)H(2) |
|----------|--------------|----------|
| W(1)     | 1.0          | _        |
| A(0)H(2) | 📁 ρ(0.7500) | 1.0      |
