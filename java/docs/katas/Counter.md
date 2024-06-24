# Counter: Pythonize your Java!! (Java)

> https://www.codewars.com/kata/5a3832c18882f32aca0000d2/Java

## Description:

Python is well known for its aspect "battery included", meaning that it provides A LOT of useful tools for almost anything you'd want to do. Using currently this language, there is a line you write at most one time per day (especially on Codewars):

from collections import Counter

And there it is, your beautiful tool.

I had a dream...

Well, yeah... Doing Java, I often end up thinking "That's a Counter thing!! ...Damn, I'm doing Java... :/ ".

And now that's enough. I WANT MY COUNTER IN JAVA TOO!!   Would you help me with that, please? :)

Note: parts of the current description uses bits of the original Python Counter API

TASK

Basically, a Counter<Something> object is a HashMap<Something,Long>, with all the associated methods (meaning: do not reinvent the wheel, your class should extend that), representing the number of times any "Something" (a hashable object or a primitive) occurred in a data structure. Note that the "counts" (the values of the Counter, actually) will always be of type Long.

    Your object will have to count things in arrays, streams, lists, ...
    one of its most important behavior is that it uses the default value 0L instead of throwing an error when one tries to access to or to update a nonexistent key.
    In addition to the behaviors inherited from HashMap, you'll have to add some new ones and override some of the inherited methods.

For example, Counter.of("abracadabra") will build a Counter<String> instance: Counter({"a"=5L, "b"=2L, "c"=1L, "d"=1L, "r"=2L}).

And I need you to implement it as much as possible in a user friendly manner, meaning you'll have to implement lots and lots of variants of some functions, to handle arrays of any kind of primitives, so try to reduce the boiler plate as much as you can... (let's say I want to put it in a library later. Don't worry, you'll be credited for the work. Too... ;p Even if you didn't put the docstrings in there... ;) )

Contract overview

    The Counter object can count any kind of (hashable) data.
    All methods inherited from HashMap have to be left as they are, except for the override of the get and toString methods.
    Mutators will have to be defined (push, all the variants of pushAll), using different kind of arguments. These will mutate the current instance.
    This Counter object will provide almost all the behaviors it holds in Python (API reference). For instance:
        It will be possible to do some mathematical operations with Counter objects, providing multisets (counters with only entries having their value greater than zero. Note that only multisets are subject to eliminate non positive entries): addition, subtraction, set union, set intersection, ... (see below). All of these operations will deliver a fresh Counter instance.
        The method elements and its variant elementsAsList will deliver an unordered Stream/List repeating each element as many times as its count (see below for edge cases).
        mostCommon and other variants will deliver a Stream/List of Map.Entry objects, ordered by count values (see below for details).
    We will add one tiny behavior that Python's Counter does not provide: the mul(long n) method which will multiply all the values of a Counter by n, once again returning a brand new (shiny?) Counter instance.
    About inheritance, keep in mind that Counter instances should be able to count instances of different kinds of subclasses if the Counter itself is declared appropriately. An example is better than a long explaination: see last section of the description.


Disclaimer...

The most annoying thing about this kata and Java is: as long as you won't get the correct declarations of the methods, the code won't even compile. To ease a bit your journey through this hell (... x) ) the sample tests contain a bit of each requirement, and especially, the one about the inheritance of the objects you have to count (see end of the description) is commented out for now. Use it as you like.

Constructors

Your implementation will have to handle the basic Map-related constructors and some new ones:

new Counter()
new Counter(int capacity)
new Counter(int capacity, float loadFactor)
new Counter(Map<T,Long> map)                  Where "T" can be any kind of hashable objects (but not primitives)

new Counter(T[] arr)                          Same here about the "T": any kind of hashable object that isn't primitive
new Counter(Collection<T> coll)
new Counter(Stream<T> stream)

The four first behaves as regular mappings constructors (new empty objects and copy of map as Counter object).
The other ones will actually count the elements of their input and create the corresponding Counter object.

Example:

Counter<String> cnt = new Counter(Arrays.asList("a","a","a","hello","b","b"));
    -> cnt <=> Counter({"a"=3L, "b"=2L, "hello"=1L})


Static builders

To make the Counter easy to use, it has to handle the counting with arrays of primitives too. To achieve that, you'll have to implement these specific static methods:

static Counter<Boolean>   of(boolean[] arr)
static Counter<Byte>      of(byte[]    arr)
static Counter<Character> of(char[]    arr)
static Counter<Double>    of(double[]  arr)
static Counter<Float>     of(float[]   arr)
static Counter<Integer>   of(int[]     arr)
static Counter<Long>      of(long[]    arr)
static Counter<Short>     of(short[]   arr)

static Counter<String>    of(String    s  )

Note that the last one with String argument will have to count the characters in this string but considering them as strings of length 1 (example: Counter.of("aaacbb") gives Counter({"a"=3L, "b"=2L, "c"=1L}))

Notes:

    It won't be tested, but you can implement all the methods except the one about String in a way that would allow to use them like the Stream.of method: Counter<Integer> cnt = Counter.of(1,1,2,1,5,2,2,1);
    The kata was originally designed with Java 8, so the behavior of the static builders like Map.of(key, value, ...) that appeared in Java 11 are ignored here (meaning: not tested).


Mutators

// Instance methods:
public void push(T elt)                   // increment by one 
public void push(T elt, long val)         // increment by val

public void pushAll(Collection<T> coll)
public void pushAll(T[] arr)              
public void pushAll(Stream<T> stream)

public void pushAll(Map<T,Long> other)

// Static methods:
static void pushAll (Counter<Boolean>   cnt, boolean[] arr)
static void pushAll (Counter<Byte>      cnt, byte[]    arr)
static void pushAll (Counter<Character> cnt, char[]    arr)
static void pushAll (Counter<Double>    cnt, double[]  arr)
static void pushAll (Counter<Float>     cnt, float[]   arr)
static void pushAll (Counter<Integer>   cnt, int[]     arr)
static void pushAll (Counter<Long>      cnt, long[]    arr)
static void pushAll (Counter<Short>     cnt, short[]   arr)

static void pushAll (Counter<String>    cnt, String    s  )

The push method increments the count of elt by one.
All the variants of pushAll increment by one the count of each element their second argument holds.
The last static version with the String argument will update the Counter built with the related static method (see static builders above).

Examples:

Counter c = new Counter(),
        d = new Counter();
List<Integer> lst = Arrays.asList(1,1,2,1,3,4);

lst.forEach( n -> c.push(n) );
d.pushAll(lst);

Leads to: c = Counter({1=3L, 2=1L, 3=1L, 4=1L})
          c.equals(d) == true

On the other end:

public void push(T elt, long val)

public void pushAll(Map<T,Long> other)

push(T elt, long value) increment the key elt with value, and puhsAll(Map<T,Long> other) increments the key elements of the cnt object with the associated values in other.

Observers

You'll have to override these methods:

    toString(), will return a string formatted like the following: "Counter({1=3, 2=1, 3=1, 4=1})". Order of the entries being compliant with the toString method as described in the AbstractMap class (see Java api, for what it could worth, considering our problem...).
    get(Object key), so that the method has the expected behavior for a Counter object (default value). Note that a missing key in the Counter is not created if called with this getter.


And you will implement the following methods too:

elements()
elementsAsList()

elements return a stream of elements, each repeating as many times as its count.
Elements are returned in arbitrary order and if an element’s count is less than one, it will be ignored.

elementsAsList does the same, but returning a List of objects.

Counter({"a"=4L, "b"=2L, "c"=0L, "d"=-2L}).elements()
                                          .sorted()
                                          .collect(Collectors.toList())

The example above would lead to the following list: ["a","a","a","a","b","b"].

mostCommon(int n)
mostCommonAsList(int n)

mostCommon()
mostCommonAsList()

mostCommon(int n) return a stream of the n most common elements and their counts (as a Stream<Map.Entry<T,Long>>) from the most common to the least. Elements with equal counts are ordered arbitrarily. If n is greater than the number of entries, just return the full list. If n is negative, throw an IllegalArgumentException.
mostCommonAsList(int n) does the same, but returning a List<Map.Entry> instead.

The two versions of these methods without arguments will return the Map.Entry stream/List of all the elements of the Counter.

Example:
Counter.of("abracadabra d").mostCommon(3) COULD return a List<Map.Entry<String,Long>> equivalent to the following: [("a", 5), ("r", 2), ("b", 2)]

Mutlisets/arithmetic operations

Several mathematical operations are provided for combining Counter objects to produce multisets (counters that have counts greater than zero). Addition and subtraction combine counters by adding or subtracting the counts of corresponding elements. Intersection and union return the minimum and maximum of corresponding counts. Each operation can accept inputs with signed counts, but the output will exclude results with counts of zero or less (reminder: these operations return new Counter instances).

Counter c = new Counter("aaab");
Counter d = new Counter("abb");

c.add(d)        ->  Counter({"a": 4, "b": 3})
c.sub(d)        ->  Counter({"a": 2})
c.intersect(d)  ->  Counter({"a": 1, "b": 1})
c.union(d)      ->  Counter({"a": 3, "b": 2})

The Counter object now needs only two last methods, each returning new Counter instances:

    The subtract method, which is a variant of the sub one, making the difference of the counts but this time, keeping the negative and zero values. Again, return a fresh Counter instance (even if, in Python, this method does actually mutate the original Counter instance).
    And finally, mul(long n) that will multiply all the values in the Counter by n, keeping negative and zero values.

Counter e = c.subtract(d);  
    ->  Counter({"a": 2, "b": -1])

e.mul(10);
    ->  Counter({"a": 20, "b": -10])


Management of the inheritance of the "counted objects"

Your class will have to handle the kind of scenario below:

/* Classes declaration: 
 * ********************/
 
class Person {

    private final String name;

    public Person(String name) { this.name = name;}

    public String getName() { return name; }

    @Override public boolean equals(Object obj) { 
        return (obj instanceof Person) && (Objects.equals(name, ((Person)obj).name));  
    }
    @Override public int hashCode() {return Objects.hashCode(name);}
}

class President extends Person   { public President(String name)   { super(name); }}
class CodeWarrior extends Person { public CodeWarrior(String name) { super(name); }}



/* Example of use: 
 * ***************/


Person      justAPerson = new Person("John Doe");
President   current     = new President("POTUS D.T.");
President   bush        = new President("George Anyofthem Bush");
CodeWarrior b4b         = new CodeWarrior("B4B");

Counter<Person>    people     = new Counter<>(Arrays.asList(justAPerson));
Counter<President> presidents = new Counter<>();
presidents.push(bush, 2);
presidents.push(current);

Counter<CodeWarrior> codeWarriors = new Counter<>(Arrays.asList(b4b).stream());

Counter<Person> combined = people.add(presidents);
combined.pushAll(codeWarriors);

About that inheritance thing, the method listed below are to be taken care of:

    The 3 related constructors (reminder: the one with array of object "isn't your concern")
    Same for pushAll
    The multisets methods.

There will be some other similar scenarios to check all of these methods.

