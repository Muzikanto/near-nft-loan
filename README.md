## Navigation

### First
- sh nft/dev-deploy.sh (развернуть в тестнете)
- sh nft/dev-init.sh (инициализировать в тестнете)
- sh loan/dev-deploy.sh (развернуть в тестнете)
- sh loan/dev-init.sh (инициализировать в тестнете)
- обновить NFT_CONTRACT и LOAN_CONTRACT в env файлах

### Add Nft to whitelist
- sh loan/loan_update_nft_price.sh (обновить цену и процент от цены нфт который остается в смарт контракте)
- sh loan/loan_nft_whitelist_add.sh (добавить нфт в whitelist)
- sh loan/loan_nft_whitelist.sh (список whitelist)

### Loan Nft
- обновить id в следующих файлах (для тестирования)
- sh /nft/mint.sh (создать nft)
- sh /nft/nft_approve.sh (апрув лэндингового контракта)
- sh /loan/loan_nft.sh (отправляем нфт и получаем займ)
- sh /loan/loan_rest_by_id.sh (смотрим сколько нужно выплатить чтобы погасить займ)
- sh /loan/loan_nft_pay.sh (выплатить займ за нфт)
- sh /loan/loan_nft_claim.sh (вернуть нфт, если займ выплачен)

### Liquidity provider
- sh /loan/loan_deposit.sh (отправить деньги в ликвидность)
- sh /loan/loan_withdraw.sh (вывести часть денег)
- sh /loan/loan_withdraw_all.sh (вывести все деньги)
- sh /loan/loan_balance_of.sh (просмотр баланса)
- sh /loan/loan_reward_of.sh (просмотр баланса ревардов)
- sh /loan/loan_reward_unclaimed_of.sh
- sh /loan/loan_reward_claimed_of.sh
- sh /loan/loan_claim_reward.sh (получить реварды)

