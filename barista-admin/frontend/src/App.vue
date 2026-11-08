<template>
  <div class="flex h-screen bg-gray-100 font-sans text-gray-900">
    <!-- Фиксированный сайдбар -->
    <aside class="w-64 bg-slate-900 text-white flex flex-col fixed inset-y-0 left-0 z-50">
      <div class="p-6 text-2xl font-bold tracking-wider text-amber-500 border-b border-slate-800">
        BARISTA B2B
      </div>
      <nav class="flex-1 px-4 py-6 space-y-2">
        <button 
          @click="currentTab = 'products'"
          :class="['w-full text-left px-4 py-3 rounded-lg transition font-medium', currentTab === 'products' ? 'bg-amber-600 text-white' : 'text-slate-300 hover:bg-slate-800']">
          📦 Склад (Товары)
        </button>
        <button 
          @click="currentTab = 'companies'"
          :class="['w-full text-left px-4 py-3 rounded-lg transition font-medium', currentTab === 'companies' ? 'bg-amber-600 text-white' : 'text-slate-300 hover:bg-slate-800']">
          🏢 Клиенты (Юрлица)
        </button>
        <button 
          @click="currentTab = 'machines'"
          :class="['w-full text-left px-4 py-3 rounded-lg transition font-medium', currentTab === 'machines' ? 'bg-amber-600 text-white' : 'text-slate-300 hover:bg-slate-800']">
          ☕ Кофемашины & QR
        </button>
        <button 
          @click="currentTab = 'orders'"
          :class="['w-full text-left px-4 py-3 rounded-lg transition font-medium', currentTab === 'orders' ? 'bg-amber-600 text-white' : 'text-slate-300 hover:bg-slate-800']">
          🛒 Новый заказ
        </button>
      </nav>
      <div class="p-4 border-t border-slate-800 text-xs text-slate-500 text-center">
        Barista Admin Panel v1.0
      </div>
    </aside>

    <!-- Основной контент -->
    <main class="flex-1 ml-64 p-8 overflow-y-auto">
      <!-- Уведомления / Ошибки -->
      <div v-if="errorMsg" class="mb-4 p-4 bg-red-100 border border-red-400 text-red-700 rounded-lg">
        {{ errorMsg }}
      </div>
      <div v-if="successMsg" class="mb-4 p-4 bg-green-100 border border-green-400 text-green-700 rounded-lg">
        {{ successMsg }}
      </div>

      <!-- ВКЛАДКА 1: ТОВАРЫ -->
      <section v-if="currentTab === 'products'">
        <div class="flex justify-between items-center mb-6">
          <h1 class="text-2xl font-bold">Управление складом</h1>
        </div>

        <!-- Форма добавления -->
        <div class="bg-white p-6 rounded-xl shadow-sm mb-6 border border-gray-200">
          <h2 class="text-lg font-semibold mb-4">Добавить новый товар</h2>
          <form @submit.prevent="createProduct" class="grid grid-cols-1 md:grid-cols-5 gap-4">
            <input v-model="newProduct.name" placeholder="Название" required class="p-2 border rounded-lg" />
            <input v-model="newProduct.sku" placeholder="SKU (артикул)" required class="p-2 border rounded-lg" />
            <input v-model.number="newProduct.price" type="number" step="0.01" placeholder="Цена (руб)" required class="p-2 border rounded-lg" />
            <input v-model.number="newProduct.stock_quantity" type="number" placeholder="Количество" required class="p-2 border rounded-lg" />
            <button type="submit" class="bg-amber-600 text-white font-medium p-2 rounded-lg hover:bg-amber-700 transition">Добавить</button>
          </form>
        </div>

        <!-- Таблица товаров -->
        <div class="bg-white rounded-xl shadow-sm overflow-hidden border border-gray-200">
          <table class="w-full text-left border-collapse">
            <thead class="bg-gray-50 border-b border-gray-200 text-gray-600 uppercase text-xs">
              <tr>
                <th class="p-4">ID</th>
                <th class="p-4">Название</th>
                <th class="p-4">SKU</th>
                <th class="p-4">Цена</th>
                <th class="p-4">На складе</th>
                <th class="p-4">В резерве</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-200">
              <tr v-for="p in products" :key="p.id" class="hover:bg-gray-50">
                <td class="p-4">{{ p.id }}</td>
                <td class="p-4 font-medium">{{ p.name }}</td>
                <td class="p-4 text-gray-500">{{ p.sku }}</td>
                <td class="p-4">{{ p.price }} ₽</td>
                <td class="p-4 font-semibold text-blue-600">{{ p.stock_quantity }}</td>
                <td class="p-4 font-semibold text-amber-600">{{ p.reserved_quantity }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      <!-- ВКЛАДКА 2: КЛИЕНТЫ -->
      <section v-if="currentTab === 'companies'">
        <div class="flex justify-between items-center mb-6">
          <h1 class="text-2xl font-bold">Клиенты (Юрлица)</h1>
        </div>

        <div class="bg-white p-6 rounded-xl shadow-sm mb-6 border border-gray-200">
          <h2 class="text-lg font-semibold mb-4">Добавить клиента</h2>
          <form @submit.prevent="createCompany" class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <input v-model="newCompany.name" placeholder="Название компании" required class="p-2 border rounded-lg" />
            <input v-model="newCompany.inn" placeholder="ИНН" required class="p-2 border rounded-lg" />
            <input v-model="newCompany.kpp" placeholder="КПП" class="p-2 border rounded-lg" />
            <input v-model="newCompany.legal_address" placeholder="Юр. адрес" class="p-2 border rounded-lg" />
            <input v-model="newCompany.contact_person" placeholder="Контактное лицо" class="p-2 border rounded-lg" />
            <input v-model="newCompany.phone" placeholder="Телефон" class="p-2 border rounded-lg" />
            <button type="submit" class="col-span-full bg-amber-600 text-white font-medium p-2 rounded-lg hover:bg-amber-700 transition">Зарегистрировать клиента</button>
          </form>
        </div>

        <div class="bg-white rounded-xl shadow-sm overflow-hidden border border-gray-200">
          <table class="w-full text-left border-collapse">
            <thead class="bg-gray-50 border-b border-gray-200 text-gray-600 uppercase text-xs">
              <tr>
                <th class="p-4">ID</th>
                <th class="p-4">Компания</th>
                <th class="p-4">ИНН / КПП</th>
                <th class="p-4">Контакт</th>
                <th class="p-4">Телефон</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-200">
              <tr v-for="c in companies" :key="c.id" class="hover:bg-gray-50">
                <td class="p-4">{{ c.id }}</td>
                <td class="p-4 font-medium">{{ c.name }}</td>
                <td class="p-4 text-gray-500">{{ c.inn }} / {{ c.kpp || '-' }}</td>
                <td class="p-4">{{ c.contact_person || '-' }}</td>
                <td class="p-4">{{ c.phone || '-' }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      <!-- ВКЛАДКА 3: КОФЕМАШИНЫ & QR -->
      <section v-if="currentTab === 'machines'">
        <div class="flex justify-between items-center mb-6">
          <h1 class="text-2xl font-bold">Кофемашины и QR-токенизация</h1>
        </div>

        <div class="bg-white p-6 rounded-xl shadow-sm mb-6 border border-gray-200">
          <h2 class="text-lg font-semibold mb-4">Установить аппарат</h2>
          <form @submit.prevent="createMachine" class="grid grid-cols-1 md:grid-cols-4 gap-4">
            <select v-model.number="newMachine.company_id" class="p-2 border rounded-lg" required>
              <option disabled value="">Выберите клиента</option>
              <option v-for="c in companies" :value="c.id">{{ c.name }}</option>
            </select>
            <input v-model="newMachine.model" placeholder="Модель аппарата" required class="p-2 border rounded-lg" />
            <input v-model="newMachine.serial_number" placeholder="Серийный номер" required class="p-2 border rounded-lg" />
            <input v-model="newMachine.installation_address" placeholder="Адрес установки" required class="p-2 border rounded-lg" />
            <button type="submit" class="col-span-full bg-amber-600 text-white font-medium p-2 rounded-lg hover:bg-amber-700 transition">Добавить и сгенерировать QR</button>
          </form>
        </div>

        <div class="bg-white rounded-xl shadow-sm overflow-hidden border border-gray-200">
          <table class="w-full text-left border-collapse">
            <thead class="bg-gray-50 border-b border-gray-200 text-gray-600 uppercase text-xs">
              <tr>
                <th class="p-4">ID</th>
                <th class="p-4">Клиент</th>
                <th class="p-4">Модель / Серийник</th>
                <th class="p-4">Адрес</th>
                <th class="p-4">QR Токен</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-200">
              <tr v-for="m in machines" :key="m.id" class="hover:bg-gray-50">
                <td class="p-4">{{ m.id }}</td>
                <td class="p-4 font-medium">{{ getCompanyName(m.company_id) }}</td>
                <td class="p-4">{{ m.model }} <span class="text-xs text-gray-400">({{ m.serial_number }})</span></td>
                <td class="p-4 text-gray-600">{{ m.installation_address }}</td>
                <td class="p-4 font-mono text-xs bg-gray-50 text-indigo-600">{{ m.qr_token }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      <!-- ВКЛАДКА 4: ЗАКАЗЫ -->
      <section v-if="currentTab === 'orders'">
        <div class="flex justify-between items-center mb-6">
          <h1 class="text-2xl font-bold">Оформление заказа (Резерв склада)</h1>
        </div>

        <div class="bg-white p-6 rounded-xl shadow-sm mb-6 border border-gray-200">
          <form @submit.prevent="createOrder" class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Клиент</label>
                <select v-model.number="orderForm.company_id" class="w-full p-2 border rounded-lg" required>
                  <option disabled value="">Выберите компанию</option>
                  <option v-for="c in companies" :value="c.id">{{ c.name }}</option>
                </select>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Привязка к аппарату (опционально)</label>
                <select v-model.number="orderForm.machine_id" class="w-full p-2 border rounded-lg">
                  <option :value="null">Без привязки (складской заказ)</option>
                  <option v-for="m in filteredMachines" :value="m.id">{{ m.model }} — {{ m.installation_address }}</option>
                </select>
              </div>
            </div>

            <div class="border-t pt-4">
              <h3 class="text-md font-semibold mb-2">Позиции заказа</h3>
              <div v-for="(item, index) in orderForm.items" :key="index" class="flex gap-4 mb-2 items-center">
                <select v-model.number="item.product_id" class="flex-1 p-2 border rounded-lg" required>
                  <option disabled value="">Выберите товар</option>
                  <option v-for="p in products" :value="p.id">{{ p.name }} (Доступно: {{ p.stock_quantity - p.reserved_quantity }}) — {{ p.price }} ₽</option>
                </select>
                <input v-model.number="item.quantity" type="number" min="1" placeholder="Кол-во" required class="w-32 p-2 border rounded-lg" />
                <button type="button" @click="orderForm.items.splice(index, 1)" class="text-red-500 hover:text-red-700 font-bold px-2">✕</button>
              </div>
              <button type="button" @click="orderForm.items.push({ product_id: '', quantity: 1 })" class="mt-2 text-sm text-amber-600 font-medium hover:underline">+ Добавить товар</button>
            </div>

            <button type="submit" class="w-full bg-amber-600 text-white font-medium p-3 rounded-lg hover:bg-amber-700 transition mt-4">Оформить заказ и зарезервировать склад</button>
          </form>
        </div>
      </section>
    </main>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'

const currentTab = ref('products')
const errorMsg = ref('')
const successMsg = ref('')

const products = ref([])
const companies = ref([])
const machines = ref([])

const newProduct = ref({ name: '', sku: '', price: 0, stock_quantity: 0 })
const newCompany = ref({ name: '', inn: '', kpp: '', legal_address: '', contact_person: '', phone: '' })
const newMachine = ref({ company_id: '', model: '', serial_number: '', installation_address: '' })

const orderForm = ref({
  company_id: '',
  machine_id: null,
  items: [{ product_id: '', quantity: 1 }]
})

const API_BASE = '/api'

const fetchAll = async () => {
  try {
    const [pRes, cRes, mRes] = await Promise.all([
      fetch(`${API_BASE}/products`),
      fetch(`${API_BASE}/companies`),
      fetch(`${API_BASE}/machines`)
    ])
    products.value = await pRes.json()
    companies.value = await cRes.json()
    machines.value = await mRes.json()
  } catch (e) {
    errorMsg.value = 'Ошибка загрузки данных с бэкенда'
  }
}

onMounted(fetchAll)

const createProduct = async () => {
  errorMsg.value = ''
  successMsg.value = ''
  const res = await fetch(`${API_BASE}/products`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(newProduct.value)
  })
  if (res.ok) {
    successMsg.value = 'Товар успешно создан!'
    newProduct.value = { name: '', sku: '', price: 0, stock_quantity: 0 }
    fetchAll()
  } else {
    errorMsg.value = 'Ошибка при создании товара'
  }
}

const createCompany = async () => {
  errorMsg.value = ''
  successMsg.value = ''
  const res = await fetch(`${API_BASE}/companies`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(newCompany.value)
  })
  if (res.ok) {
    successMsg.value = 'Клиент успешно зарегистрирован!'
    newCompany.value = { name: '', inn: '', kpp: '', legal_address: '', contact_person: '', phone: '' }
    fetchAll()
  } else {
    errorMsg.value = 'Ошибка при создании клиента'
  }
}

const createMachine = async () => {
  errorMsg.value = ''
  successMsg.value = ''
  const res = await fetch(`${API_BASE}/machines`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(newMachine.value)
  })
  if (res.ok) {
    successMsg.value = 'Кофемашина зарегистрирована и QR-токен сгенерирован!'
    newMachine.value = { company_id: '', model: '', serial_number: '', installation_address: '' }
    fetchAll()
  } else {
    errorMsg.value = 'Ошибка при регистрации машины'
  }
}

const createOrder = async () => {
  errorMsg.value = ''
  successMsg.value = ''
  const res = await fetch(`${API_BASE}/orders`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(orderForm.value)
  })
  if (res.ok) {
    successMsg.value = 'Заказ успешно создан, склад зарезервирован!'
    orderForm.value = { company_id: '', machine_id: null, items: [{ product_id: '', quantity: 1 }] }
    fetchAll()
  } else if (res.status === 409) {
    errorMsg.value = 'Недостаточно товара на складе для резервирования (конфликт остатков)'
  } else {
    errorMsg.value = 'Ошибка при создании заказа'
  }
}

const getCompanyName = (id) => {
  const comp = companies.value.find(c => c.id === id)
  return comp ? comp.name : 'Не указана'
}

const filteredMachines = computed(() => {
  if (!orderForm.value.company_id) return machines.value
  return machines.value.filter(m => m.company_id === orderForm.value.company_id)
})
</script>
