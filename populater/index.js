const Chance = require("chance")
const csvWriter = require('csv-writer').createObjectCsvWriter;

const userHeader = [
    { id: "id", title: "id" },
    { id: "nome", title: "nome" },
    { id: "cpf", title: "cpf" },
    { id: "cnpj", title: "cnpj" },
    { id: "email", title: "email" },
    { id: "senha", title: "senha" },
    { id: "user_type", title: "user_type" },
]

const walletHeader = [
    { id: "user_id", title: "user_id" },
    { id: "currency", title: "currency" },
    { id: "amount", title: "amount" },
]

const transferHeader = [
    { id: "payer_id", title: "payer_id" },
    { id: "payee_id", title: "payee_id" },
    { id: "amount", title: "amount" },
    { id: "wallet_payer_id", title: "wallet_payer_id" },
    { id: "wallet_payee_id", title: "wallet_payee_id" },
    { id: "currency", title: "currency" },
    { id: "status", title: "status" },
]

const generator = Chance()

const userData = []
const usersTotal = 550000
const userCpfHash = new Set()
const userCnpjHash = new Set()
const userEmailHash = new Set()

for (let index = 0; index < usersTotal; index++) {

    userData.push({
        id: index,
        nome: generator.name(),
        cpf: generateUniqueData(() => generator.string({ length: 11, numeric: true }), userCpfHash),
        cnpj: generateUniqueData(() => generator.string({ length: 14, numeric: true }), userCnpjHash),
        email: generateUniqueData(() => generator.email(), userEmailHash),
        senha: generator.hash({ length: 20 }),
        user_type: generator.pickone(["logista", "comum"]),
    })
}

createCsv("csv/users.csv", userHeader, userData)

function createCsv(path, header, data) {
    const csvWriterInstance = csvWriter({
        path: path,
        header: header
    });

    csvWriterInstance.writeRecords(data)
        .then(() => {
            console.log('CSV created');
        })
        .catch(err => {
            console.error('Error writing CSV file', err);
        });
}

function generateUniqueData(gen, hash) {
    do {
        data = gen()
    } while (hash.has(data));
    hash.add(data);
    return data;
}
