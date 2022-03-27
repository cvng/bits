# Bits Backend

## Specifications

The challenge is to create a GraphQL endpoint that have these features

- a seller can create shows with products to successively start auctions on
  during the same show) and start it when ready
- any buyer should be able to bid and write comments
- anyone should see the bids and comments in real time

The show process works like this:

The seller prepare then starts the show.

All buyers should then be able to bid during 1 minute.

If a bid occurs during the last 15 seconds of the auction, the remaining time is
reset to 15 seconds.

Each bid should update the clients with the current bit and the user who bid.

At the end of the auction, the winner should be announced to everyone

---

## Expectations

We expect you:

- to use Typescript
- to use GraphQL subscriptions for real-time data
- to have a demo environment where we can show how the system works. We will not
  evaluate the look of it. It can apollo studio, a webpage, an app or anything
  else. See
  [assets](https://www.notion.so/Bits-Backend-Technical-Test-437bfdc242014f83b03b10f42f0ef7e2)
  for an example.

You are free to choose:

- the GraphQL tools you want
- code first or schema first approach

---

## Assets

### **HLS video stream**

```json
"https://stream.mux.com/02cDqggWRQ2GkJskapoOW2OZq7I4NGL2y8aFApetXkBA.m3u8"
```

### **Figma Designs**

[https://www.figma.com/embed?embed_host=notion&url=https%3A%2F%2Fwww.figma.com%2Ffile%2Ft3rjw1SXLq2OnSM9hJgd04%2FTechnical-Test---Ressources%3Fnode-id%3D0%253A1](https://www.figma.com/embed?embed_host=notion&url=https%3A%2F%2Fwww.figma.com%2Ffile%2Ft3rjw1SXLq2OnSM9hJgd04%2FTechnical-Test---Ressources%3Fnode-id%3D0%253A1)

<aside>
💡 This design is not the one expected at the end of the test, use it as you wish... Size, icons, colors, etc..

</aside>

---

## General note

Do your best, take shortcuts, use libraries... We like to see creative and
different approaches and, more importantly, we are okay with imperfect results.

---

## Contact

For any question or problem, you can contact Léandre: +33 6 31 18 82 69
