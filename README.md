# Token-2022 AMM Frontend

A beautiful, modern React frontend for the Token-2022 AMM (Automated Market Maker) with transfer hook support.

## 🚀 Features

### ✨ Modern UI/UX
- **Glass Morphism Design**: Beautiful glass-effect cards and components
- **Smooth Animations**: Framer Motion powered animations and transitions
- **Responsive Design**: Works perfectly on desktop, tablet, and mobile
- **Dark Theme**: Elegant dark gradient background with purple accents

### 🔧 Core Functionality
- **Token Creation**: Create Token-2022 tokens with transfer hooks
- **Pool Management**: Create and manage liquidity pools
- **Trading Interface**: Swap tokens with real-time pricing and transfer hook validation
- **Whitelist Management**: Manage transfer hook whitelists

### 🛡️ Security Features
- **Transfer Hook Integration**: All transfers validated by custom transfer hooks
- **Whitelist Support**: Restrict transfers to approved addresses
- **Secure Trading**: AMM swaps respect transfer hook restrictions

## 🎨 Design System

### Colors
- **Primary**: Blue to Purple gradients
- **Secondary**: Slate grays with white transparency
- **Success**: Green accents
- **Warning**: Yellow accents
- **Error**: Red accents

### Components
- **Glass Effect**: `bg-white/10 backdrop-blur-lg border border-white/20`
- **Cards**: Rounded corners with glass effect and shadows
- **Buttons**: Gradient backgrounds with hover animations
- **Inputs**: Glass effect with focus states

## 📱 Pages

### 1. Token Creation
- Create Token-2022 mints with transfer hooks
- Configure transfer hook settings
- Enable/disable whitelist validation
- Real-time form validation

### 2. Pool Management
- Create new liquidity pools
- Manage existing pools
- View pool statistics
- Add/remove liquidity

### 3. Trading Interface
- Swap tokens with real-time pricing
- Transfer hook validation integration
- Market overview with live data
- Recent activity feed

### 4. Whitelist Management
- Add addresses to whitelist
- Manage existing whitelist entries
- Search and filter functionality
- Status tracking (active, pending, expired)

## 🛠️ Technology Stack

- **React 18** with TypeScript
- **Tailwind CSS** for styling
- **Framer Motion** for animations
- **Lucide React** for icons
- **Headless UI** for accessible components

## 🚀 Getting Started

### Prerequisites
- Node.js 16+ 
- npm or yarn

### Installation
```bash
# Clone the repository
git clone <repository-url>
cd token2022-amm-frontend

# Install dependencies
npm install

# Start development server
npm start
```

### Available Scripts
```bash
npm start          # Start development server
npm run build      # Build for production
npm run test       # Run tests
npm run eject      # Eject from Create React App
```

## 🎯 Hackathon Project

This frontend is part of the **"Make Token-2022 with Transfer Hooks Tradable on Solana AMMs"** hackathon project.

### Problem Solved
- **Token-2022 Integration**: Full support for Token-2022 tokens with transfer hooks
- **AMM Compatibility**: Seamless integration with automated market makers
- **Transfer Hook Validation**: All transfers validated by custom logic
- **User-Friendly Interface**: Beautiful UI for complex DeFi operations

### Key Innovations
1. **Transfer Hook Integration**: First AMM to support Token-2022 transfer hooks
2. **Whitelist Management**: Visual interface for managing transfer restrictions
3. **Secure Trading**: All swaps respect transfer hook validations
4. **Modern UX**: Professional-grade interface for DeFi operations

## 🔗 Backend Integration

This frontend connects to the Token-2022 AMM smart contracts:
- **AMM Program**: `BXkgQBaKiJS7AunZPYAHGqQ5qKz6gJ7vTjedNYM1FUkU`
- **Transfer Hook Program**: `E24fiZAUbQNAwEkCcyzm9b4UFi4hhPdqMJgN9ZqntJgq`

## 📸 Screenshots

The application features:
- Beautiful glass morphism design
- Smooth animations and transitions
- Responsive layout for all devices
- Real-time data visualization
- Professional DeFi interface

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is part of a hackathon submission and is open source.

## 🙏 Acknowledgments

- Solana Labs for Token-2022
- Anchor Framework for smart contract development
- Framer Motion for animations
- Tailwind CSS for styling
- Lucide for beautiful icons

---

**Built with ❤️ for the Solana Hackathon**
