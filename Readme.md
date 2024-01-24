# Computer Science Homework (EF-KUE)

Task: Determine Email Legitimacy

Recently, I watched a video about how autocorrect works, and I thought that the same principle could be applied to spam detection. In my experience, most spam emails involve phishing attempts, where they try to trick users into clicking on malicious links.

My approach is to calculate the Levenshtein distance for each word against a list of popular URLs. If the distance is not zero but also not greater than 3, it is highly likely that the URL is attempting to deceive people.

I haven't conducted extensive research beyond understanding how to implement the algorithem, so I'm no sure if this approach is a common practice in spam detection.
