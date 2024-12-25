import { PrismaClient } from '@prisma/client'

const prisma = new PrismaClient()

async function main() {
  const user = await prisma.guestbookEntry.create({
    data: {
      name: "Melody",
      date: new Date(),
      website: "melody-is.gay",
      text: "Hello!",
    }
  })
}

main()
  .then(async () => {
    await prisma.$disconnect()
  })
  .catch(async (e) => {
    console.error(e)
    await prisma.$disconnect()
  })