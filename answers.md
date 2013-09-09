Title: Problem Set 1 Answers
Author: Jake Kenneally


1. Mozilla/5.0 -- browser is based on Mozilla or is Mozilla compatible, version 5.0
(X11; Linux i686) -- using the X Windowing System; coming from a linux OS on an i686 processor
AppleWebKit/537.36 (KHTML, like Gecko) -- WebKit is a set of tools for browsers to render webpages, the version number is 537.36; using KHTML, a version of HTML made by KDE, and it is like Gecko, which was considered to be a great engine developed by Mozila
Chrome/29.0.1547.57 -- is a chrome browser with version 29.0.1547.57
Safari/537.36 -- based on Safari, build 537.36

2. Rust probably thinks this is unsafe due to concurrency issues, and it attempts to prevent those entirely by declaring the operation unsafe. Basically, it doesn't want the issue of two functions using visitor_count at the same time, but having it change it's value in the process of accessing it. This can cause delays in functions being executed, and it can cause issues with intended values being assigned incorrectly.

