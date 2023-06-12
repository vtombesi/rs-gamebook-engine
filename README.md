# rs-gamebook-engine

```json
{
  "start_page": 1,
  "pages": [
    {
      "id": 1,
      "text": "Sei un valoroso guerriero nel regno di Eldoria. Hai dimostrato la tua abilità in molte battaglie e ora ti è stata assegnata una missione cruciale. La principessa Elysia è stata rapita dal malvagio stregone Morlog. Devi salvarla e riportarla sana e salva al regno di Arathor. Ti trovi ora all'ingresso della pericolosa Caverna dell'Oscurità, dove si dice che Morlog abbia stabilito la sua dimora. Sei pronto ad affrontare questa sfida?",
      "options": [
        {
          "text": "Avventurati nella Caverna dell'Oscurità",
          "destination": 2
        },
        {
          "text": "Attacca il manichino",
          "destination": 3,
          "combat": {
            "name": "Manichino",
            "health": 10,
            "attacks": [
              {
                "name": "Claw",
                "damage": 5
              },
              {
                "name": "Bite",
                "damage": 3
              }
            ],
            "victory_text": "Il manichino finisce a pezzi davanti a te.",
            "defeat_text": "Il manichino prende vita e ti sconfigge. Sei stato sconfitto.",
            "loot": "Pozione curativa"
          }
        }
      ]
    }
  ]
}
```
