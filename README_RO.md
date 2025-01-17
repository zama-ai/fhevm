<p align="center">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zama-ai/fhevm/assets/157474013/b07e7e65-12b2-4048-b5de-35e169ed96e4">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/zama-ai/fhevm/assets/157474013/c0fab5b1-adef-4db4-9607-fa0a793acaf8">
  <img width=600 alt="Zama fhEVM">
</picture>
</p>

<hr/>

<p align="center">
  <a href="fhevm-whitepaper-v2.pdf"> 📃 Citește white paper-ul</a> |<a href="https://docs.zama.ai/fhevm"> 📒 Documentație</a> | <a href="https://zama.ai/community"> 💛 Suport comunitar</a> | <a href="https://github.com/zama-ai/awesome-zama"> 📚 Resurse FHE de la Zama</a>
</p>

<p align="center">
  <a href="https://github.com/zama-ai/fhevm/releases">
    <img src="https://img.shields.io/github/v/release/zama-ai/fhevm?style=flat-square"></a>
  <a href="https://github.com/zama-ai/fhevm/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-BSD--3--Clause--Clear-%23ffb243?style=flat-square"></a>
  <a href="https://github.com/zama-ai/bounty-program">
    <img src="https://img.shields.io/badge/Contribuie-Programul%20de%20Recompense%20Zama-%23ffd208?style=flat-square"></a>
  <a href="https://slsa.dev"><img alt="SLSA 3" src="https://slsa.dev/images/gh-badge-level3.svg" /></a>
</p>

## Despre

### Ce este fhEVM

**fhEVM** este o tehnologie care permite contracte inteligente confidențiale pe EVM folosind criptare complet omomorfică (FHE).

Datorită unei descoperiri în criptarea omomorfică, fhEVM de la Zama face posibilă rularea contractelor inteligente confidențiale pe date criptate, garantând atât confidențialitatea, cât și compozabilitatea, cu:

- **Criptare end-to-end a tranzacțiilor și stărilor:** Datele incluse în tranzacții sunt criptate și nu sunt niciodată vizibile nimănui.
- **Compozabilitate și disponibilitate a datelor on-chain:** Stările sunt actualizate în timp ce rămân criptate în permanență.
- **Fără impact asupra aplicațiilor dApp existente și stărilor publice:** Stările criptate coexistă alături de cele publice și nu afectează aplicațiile existente.

### Caracteristici principale

- **Integrare Solidity:** Contractele fhEVM sunt contracte simple Solidity construite folosind unelte tradiționale Solidity.
- **Experiență simplă pentru dezvoltatori:** Dezvoltatorii pot folosi tipurile de date `euint` pentru a marca părțile contractelor care trebuie să fie private.
- **Confidențialitate programabilă:** Toată logica pentru controlul accesului la stările criptate este definită de către dezvoltatori în contractele inteligente.
- **Numere întregi criptate de înaltă precizie:** Până la 256 de biți de precizie pentru numerele întregi.
- **Gamă completă de operatori:** Toți operatorii tipici sunt disponibili: `+`, `-`, `*`, `/`, `<`, `>`, `==`, ...
- **Condiționale criptate if-else:** Verificați condiții pe stări criptate.
- **PRNG on-chain:** Generează aleator securizat fără utilizarea oracolelor.
- **Decriptare configurabilă:** Decriptare cu prag, centralizată sau cu KMS.
- **Adâncime de calcul nelimitată:** Operațiuni FHE consecutive nelimitate.

_Aflați mai multe despre caracteristicile fhEVM în [documentație](https://docs.zama.ai/fhevm)._

### Cazuri de utilizare

fhEVM este construit pentru dezvoltatori pentru a scrie contracte inteligente confidențiale fără a învăța criptografie. Folosind fhEVM, puteți debloca numeroase cazuri de utilizare noi, precum DeFI, gaming și altele. De exemplu:

- **Tokenizare:** Schimbă token-uri și active reale pe blockchain fără ca alții să vadă sumele.
- **Licitații oarbe:** Licitează pe obiecte fără a dezvălui suma sau câștigătorul.
- **Jocuri on-chain:** Păstrează mutările, selecțiile, cărțile sau obiectele ascunse până când sunt gata să fie dezvăluite.
- **Vot confidențial:** Previne mita și constrângerea prin păstrarea voturilor private.
- **DID-uri criptate:** Stochează identități on-chain și generează atestări fără ZK.
- **Transferuri private:** Păstrează soldurile și sumele private, fără utilizarea mixere.

_Aflați mai multe cazuri de utilizare în [lista de exemple](https://docs.zama.ai/fhevm/tutorials/see-all-tutorials)._

## Cuprins

- **[Începe](#începe)**
  - [Instalare](#instalare)
  - [Un exemplu simplu](#un-exemplu-simplu)
- **[Resurse](#resurse)**
  - [White paper](#white-paper)
  - [Demo-uri și tutoriale](#demo-uri-și-tutoriale)
  - [Documentație](#documentație)
  - [Implementare blockchain](#implementare-blockchain)
- **[Lucru cu fhEVM](#lucru-cu-fhevm)**
  - [Ghid pentru dezvoltatori](#ghid-pentru-dezvoltatori)
  - [Citații](#citații)
  - [Contribuții](#contribuții)
  - [Licență](#licență)
- **[Suport](#suport)**

## Începe

### Instalare

```bash
# Folosind npm
npm install fhevm

# Folosind Yarn
yarn add fhevm

# Folosind pnpm
pnpm add fhevm
```

### Un exemplu simplu

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

import "fhevm/lib/TFHE.sol";
import "fhevm/config/ZamaFHEVMConfig.sol";

contract Counter is SepoliaZamaFHEVMConfig {
  euint8 internal counter;

  constructor() {
    counter = TFHE.asEuint8(0);
    TFHE.allowThis(counter);
  }

  function add(einput valueInput, bytes calldata inputProof) public {
    euint8 value = TFHE.asEuint8(valueInput, inputProof);
    counter = TFHE.add(counter, value);
    TFHE.allowThis(counter);
  }
}
```

_Mai multe exemple sunt disponibile [aici](https://docs.zama.ai/fhevm/tutorials/see-all-tutorials)._

## Resurse

### **White Paper**

Descoperă tehnologia din spatele fhEVM cu white paper-ul nostru detaliat:  
👉 [**Contracte inteligente confidențiale pe EVM folosind criptare complet omomorfică**](https://github.com/zama-ai/fhevm/blob/main/fhevm-whitepaper-v2.pdf)

---

### **Demo-uri și tutoriale**

Accesați o colecție curată de demo-uri și tutoriale pas cu pas pentru a vă ghida în călătoria de dezvoltare:  
🔗 [**Vizitați pagina de tutoriale**](https://docs.zama.ai/fhevm/tutorials/see-all-tutorials)

---

### **Documentație**

Stăpâniți `fhEVM` și construiți contracte inteligente folosind aceste resurse:

- 📘 [**Documentație completă fhEVM**](https://docs.zama.ai/fhevm)  
  Aflați mai multe din ghidul detaliat Zama pentru utilizarea la potențial maxim a fhEVM.

- 🤖 [**Zama Solidity Developer (Model modificat ChatGPT)**](https://chatgpt.com/g/g-67518aee3c708191b9f08d077a7d6fa1-zama-solidity-developer)
  Accelerați dezvoltarea contractelor inteligente cu asistență bazată pe AI.

---

### **Template-uri de dezvoltare**

Începeți mai repede folosind template-uri preconfigurate pentru diverse cadre de dezvoltare:

#### **Contracte inteligente**

- 🔧 [**Template Hardhat**](https://github.com/zama-ai/fhevm-hardhat-template)  
  Testare și dezvoltare contracte inteligente - punct de intrare principal pentru dezvoltatorii care doresc să dezvolte contracte inteligente pe fhEVM.

- 💻 [**Contracte fhEVM**](https://github.com/zama-ai/fhevm-contracts)  
  Biblioteca de contracte fhEVM standardizate.

#### **Framework-uri frontend**

- 🌐 [**Template React.js**](https://github.com/zama-ai/fhevm-react-template)  
  Simplificați dezvoltarea aplicațiilor descentralizate FHE cu un template React.js curat și optimizat.

- ⚡ [**Template Next.js**](https://github.com/zama-ai/fhevm-next-template)  
  Construiți aplicații descentralizate scalabile, renderizate pe server, cu suport FHE, folosind acest template Next.js.

- 🖼️ [**Template Vue.js**](https://github.com/zama-ai/fhevm-vue-template)  
  Creați aplicații descentralizate modulare, responsive, cu capabilități FHE, folosind Vue.js.

---

### 🚀 **Lansați-vă proiectul astăzi!**

Folosiți aceste template-uri pentru a accelera procesul de dezvoltare și a vă aduce ideile la viață mai repede.

## Implementare blockchain

Pentru a integra fhevm-go într-un blockchain bazat pe EVM, urmați [Ghidul de integrare](https://docs.zama.ai/fhevm-go/getting_started/integration).

## Lucru cu fhEVM

### Ghid pentru dezvoltatori

Instalați dependențele (biblioteci Solidity și unelte de dezvoltare):

```bash
npm install
```

> **Notă:** Fișierele Solidity sunt formatate cu Prettier.

#### Generați biblioteca TFHE

```bash
npm run codegen
```

> **Atenție:** Utilizați această comandă pentru a genera cod Solidity și rezultate formate cu Prettier automat!

Fișierele generate acum (pot fi văzute în `codegen/main.ts`):

```
lib/Impl.sol
lib/TFHE.sol
contracts/tests/TFHETestSuiteX.sol
test/tfheOperations/tfheOperations.ts
```

#### Adăugarea de operatori noi

Operatorii pot fi definiți ca date în fișierul `codegen/common.ts`, iar codul generează automat suprascrieri Solidity. Testele pentru suprascrieri trebuie adăugate (sau build-ul nu trece) în fișierul `codegen/overloadsTests.ts`.

### Citații

Pentru a cita fhEVM sau whitepaper-ul în lucrări academice, folosiți următoarele înregistrări:

```text
@Misc{fhEVM,
title={{Contracte inteligente confidențiale pe EVM folosind criptare complet omomorfică}},
author={Zama},
year={2024},
note={\url{https://github.com/zama-ai/fhevm}},
}
```

```text
@techreport{fhEVM,
author = "Zama",
title = "Contracte inteligente confidențiale pe EVM folosind criptare complet omomorfică",
institution = "Zama",
year = "2024"
}
```

### Contribuții

Există două moduri de a contribui la fhEVM de la Zama:

- [Deschideți probleme](https://github.com/zama-ai/fhevm/issues/new/choose) pentru a raporta bug-uri și greșeli, sau pentru a sugera idei noi.
- Solicitați să deveniți contribuitor oficial trimițând un email la hello@zama.ai.

A deveni contribuitor aprobat implică semnarea unui Acord de Licență pentru Contributori (CLA). Doar contribuitorii aprobați pot trimite pull requests, deci asigurați-vă că luați legătura înainte de a face acest lucru!

### Licență

Acest software este distribuit sub licența **BSD-3-Clause-Clear**. Citiți [detalii aici](LICENSE).

#### FAQ

**Tehnologia Zama este gratuită pentru utilizare?**

> Bibliotecile Zama sunt gratuite pentru utilizare sub licența BSD 3-Clause Clear doar pentru dezvoltare, cercetare, prototipare și experimentare. Totuși, pentru orice utilizare comercială a codului open-source Zama, companiile trebuie să achiziționeze licența comercială a brevetului Zama.

**Ce trebuie să fac dacă vreau să folosesc tehnologia Zama în scopuri comerciale?**

> Pentru utilizarea comercială a tehnologiei Zama, trebuie să vi se acorde licența brevetului Zama. Contactați-ne la hello@zama.ai pentru mai multe informații.

**Înregistrați IP pe tehnologia dumneavoastră?**

> Da, toate tehnologiile Zama sunt brevetate.

**Puteți personaliza o soluție pentru un caz specific?**

> Suntem deschiși la colaborări pentru a avansa domeniul FHE împreună cu partenerii noștri. Dacă aveți nevoi specifice, trimiteți-ne un email la hello@zama.ai.

## Suport

<a target="_blank" href="https://community.zama.ai">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zama-ai/fhevm/assets/157474013/e249e1a8-d724-478c-afa8-e4fe01c1a0fd">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/zama-ai/fhevm/assets/157474013/a72200cc-d93e-44c7-81a8-557901d8798d">
  <img alt="Suport">
</picture>
</a>

🌟 Dacă găsiți acest proiect util sau interesant, vă rugăm să îi acordați o stea pe GitHub! Sprijinul dumneavoastră ajută la creșterea comunității și motivează dezvoltarea continuă.

<p align="right">
  <a href="#despre" > ↑ Înapoi la început </a>
</p>

