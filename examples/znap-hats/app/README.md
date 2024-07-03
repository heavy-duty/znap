# znap-dealership

This project is generated with the [create-solana-dapp](https://github.com/solana-developers/create-solana-dapp) generator.

## Getting Started

### Prerequisites

- Node v18.18.0 or higher

### Installation

#### Clone the repo

```shell
git clone <repo-url>
cd <repo-name>
```

#### Install Dependencies

```shell
npm install
```

#### Start the web app

```
npm run dev
```

## Apps

### web

This is a React app.

#### Commands

Start the web app

```shell
npm run dev
```

Build the web app

```shell
npm run build
```

---

dApp setup:

- [x] Display a list of products in the home page.
- [x] Clicking a product takes you to a product details page.
- [x] Product details display shows price and a button to buy.
- [ ] Clicking the buy button triggers a SOL transfer.
- [ ] Deploy it somewhere for free. (shuttle would be cool)

Actions setup:

- [ ] Create a shuttle axum api.
- [ ] Create a znap collection with an action buy product.
- [ ] Support dynamic metadata for buy product action.
- [ ] Customize action.json to map product pages in the site to buy product actions.
- [ ] Deploy using shuttle.