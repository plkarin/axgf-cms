# axgf-cms — textos da interface, português.
#
# QUALIDADE AUTOMÁTICA — não revisto por uma pessoa de língua materna
# portuguesa. O vocabulário genealógico tem equivalências firmadas que variam
# conforme a tradição arquivística, e esta tradução pode estar errada.
# Correcções são bem-vindas — ver CONTRIBUTING.md.
#
# Variedade: português europeu.
#
# Escolhas adoptadas (contestáveis):
#   union → união · link → vínculo · confidence → certeza
#   reliability → fiabilidade · source → fonte
#   primary source → fonte primária · occupation → ocupação
#   record → ficha · archive → arquivo · godparent → padrinho/madrinha
#   witness → testemunha · speculative → hipotético
#
# Plural: regras CLDR one / other. Nunca substituir por lógica própria.
#
# Datas: «12 de abril de 1923» — com as duas preposições e o mês em minúscula.
# A tabela dos meses vive dentro do próprio padrão da data.
#
# REGRA: este ficheiro traduz apenas a interface. Nomes, lugares, notas e
# ofícios vêm do arquivo familiar e ficam na sua língua e na sua escrita.

app-name = ax-genealogy

## Cabeçalho e rodapé

nav-tree = Árvore
nav-convert = Importar
nav-admin = Administração
nav-sign-in = Entrar
nav-sign-out = Sair
footer-open-format = O arquivo da sua família é um único ficheiro que fica consigo, escrito num formato aberto: continuará a abrir muito depois de este sítio ter desaparecido.
footer-open-format-link = Sobre o formato

## Preferências

prefs-title = Língua e aspecto
prefs-language = Língua
prefs-theme = Aspecto
prefs-background = Fundo
prefs-background-on = Um véu suave de cor por trás da página
prefs-apply = Aplicar
prefs-reviewed = revista
prefs-machine = automática, { $coverage } %
prefs-machine-complete = completa, ainda por rever
prefs-machine-title = Traduzida sem revisão por uma pessoa de língua materna. O vocabulário genealógico em particular pode estar errado: as palavras para uma união, um padrinho ou uma fonte primária mudam conforme a tradição arquivística de cada país. Correcções são bem-vindas, e o CONTRIBUTING.md diz por onde começar.

theme-light = Claro
theme-dark = Escuro
theme-system = Como o sistema
theme-high-contrast = Contraste elevado
theme-sepia = Sépia
theme-deuteranopia = Deuteranopia
theme-protanopia = Protanopia
theme-tritanopia = Tritanopia
theme-colour-blind-note = seguro para daltonismo
theme-contrast-note = contraste máximo

## Árvore

tree-title-around = À volta de { $name }
tree-title-whole = A árvore inteira
tree-lede-focused = { $ancestors ->
        [one] Um antepassado
       *[other] { $ancestors } antepassados
    }, { $descendants ->
        [one] um descendente
       *[other] { $descendants } descendentes
    } e { $spouses ->
        [one] um parceiro
       *[other] { $spouses } parceiros
    }, { $depth } gerações para cada lado.
tree-filter-label = Filtrar as fichas visíveis
tree-filter-placeholder = Escreva um nome…
tree-centre-on = Centrar em
tree-depth = Gerações para cada lado
tree-show = Mostrar
tree-hidden-notice = { $n ->
        [one] Uma pessoa é mostrada sem os seus dados
       *[other] { $n } pessoas são mostradas sem os seus dados
    }
tree-hidden-because-role = , porque a sua visibilidade está acima do que a sua conta pode ler.
tree-hidden-because-anonymous = , porque não são públicas.
tree-hidden-sign-in = Entre, se tiver conta.
tree-restricted-card = Esta ficha não lhe é visível
tree-empty = Ainda não há ninguém para desenhar.
tree-unplaced = Em nenhuma família registada

## A ficha

record-identity = Identidade
record-life-events = Factos da vida
record-family = Família
record-other-relationships = Outras relações
record-occupations = Ocupações
record-places = Lugares
record-sources-documents = Fontes e documentos
record-notes = Notas
record-history = Histórico de alterações
record-raw = Dados em bruto
record-raw-summary-note = o JSON com que esta página foi construída

record-identity-help = Cada nome registado com o seu tipo, o período em que foi usado e a fonte que o sustenta, com a escrita própria ao lado da transliteração latina onde diferem, além do género, se está vivo e a visibilidade.
record-life-events-help = Nascimento, óbito e cada facto em que esta pessoa tomou parte, por data, cada um com o seu papel — assim um casamento de que foi apenas testemunha aparece ao lado do seu próprio. Um dado sem data vai para o fim, em vez de fingir que vem primeiro.
record-family-help = Pais e irmãos, depois cada união com o seu tipo, as datas, o lugar, como terminou e os filhos por ordem de nascimento.
record-other-relationships-help = Cada vínculo com esta pessoa numa das pontas, lido do seu lado: a mesma ficha aparece como «padrinho de» de uma ponta e «afilhado de» da outra.
record-occupations-help = As ocupações como períodos sobre um mesmo eixo, de modo que dois empregos se comparem a olho; onde falta um extremo a barra fica aberta.
record-places-help = Cada lugar que esta ficha toca, com o que ali aconteceu e com a história das fronteiras que faz um lugar ter sentido ao longo do tempo.
record-sources-documents-help = Cada fonte nomeia os factos desta página que nela se apoiam, por ordem da força da prova.
record-notes-help = Notas sobre esta ficha, incluindo o texto que nenhum conversor soube interpretar e que foi guardado à letra em vez de descartado.
record-history-help = Cada alteração guardada desta ficha, a mais recente primeiro. Quem corrigiu o quê é um facto sobre as pessoas que mantêm a árvore, não sobre a família que nela está: por isso fica fora do arquivo exportado e só é mostrado aos parentes com sessão iniciada.
record-raw-help = Aqui não há nada gerado para mostrar: esta é a ficha exactamente como está guardada, até aos nomes dos campos. Se um dia tiver de ler o arquivo sem este sítio, é isto que veria.
record-help-toggle = O que esta secção mostra

record-gender = Género
record-living = Vivo
record-visibility = Visibilidade
record-yes = sim
record-no = não
record-name-type = Tipo de nome
record-name-used = Em uso
record-name-evidence = Prova
record-transliteration = Transliteração latina
record-born = Nascido/a
record-died = Falecido/a
record-parents = Pais
record-siblings = Irmãos
record-children = Filhos
record-unknown-person = [Desconhecido]
record-restricted-person = Reservada
record-restricted-title = Esta ficha não lhe é visível
record-absent-person-title = Nomeado nesta árvore mas sem ficha própria
record-confidence = Certeza
record-source = Fonte
record-download = Transferir

## Acesso

access-restricted-title = Não visível para si
access-restricted-signed-in = A visibilidade desta ficha está acima do que a sua conta pode ler. Um administrador pode mudar ou a visibilidade da ficha ou o seu papel.
access-restricted-anonymous = Esta ficha não é pública. Entre para ver se a sua conta a pode ler.
access-role-title = Não para o seu papel
access-role-admin = Esta é uma página de administrador. A sua conta pode criar e editar fichas, mas não gerir contas, apagar fichas nem exportar o arquivo.
access-role-write = A sua conta pode ler esta árvore mas não alterá-la. Um administrador pode subir o seu papel a colaborador.
access-scope-title = Fora do seu ramo
access-scope-named = A sua conta está limitada a um ramo da árvore, e esta ficha diz respeito a alguém de fora. Cada pessoa nomeada numa ficha tem de estar dentro do seu ramo: caso contrário, uma família com um parceiro de fora seria uma maneira de reescrever a filiação dessa pessoa.
access-scope-unnamed = A sua conta está limitada a um ramo da árvore, e esta ficha não nomeia ninguém com quem a confrontar. As fontes e os lugares são editados por contas com acesso à árvore inteira.

## Erros

error-not-found-title = Não encontrado
error-not-found-detail = Essa página não existe aqui.
error-no-such-person-title = Não há tal pessoa
error-no-such-person-detail = Aqui não há nenhuma pessoa com esse identificador.
error-no-such-entity-title = Não há tal elemento
error-no-such-entity-detail = Aqui não há nenhuma ficha com esse identificador.
error-deleted-while-editing = Aqui não há nenhuma ficha com esse identificador. Pode ter sido apagada enquanto a editava.
error-no-such-file-title = Não há tal ficheiro
error-no-such-file-detail = Aqui não há nenhum documento com esse identificador, ou o documento está registado sem ficheiro — um documento citado nomeia algo guardado noutro sítio.
error-not-an-image-title = Não é uma imagem
error-not-an-image-detail = Para este documento não há miniatura, porque não é uma imagem que esta versão saiba descodificar.
error-back = Voltar

## Início de sessão

login-title = Entrar
login-lede = As contas são criadas por um administrador.
login-username = Nome de utilizador
login-password = Palavra-passe
login-submit = Entrar
login-wrong = Esse nome de utilizador e essa palavra-passe não correspondem.
login-token-wrong = Esse testemunho não está correcto.
login-throttled = Demasiadas tentativas falhadas. Espere alguns minutos e tente outra vez.
login-no-accounts-title = Esta instalação ainda não tem contas.
login-no-accounts-detail = Não há aqui página de configuração de propósito: o intervalo entre a entrada em serviço e o primeiro acesso é precisamente o momento em que uma instalação está desprotegida, por isso o primeiro administrador cria-se na linha de comandos.
login-no-accounts-note = Imprime uma palavra-passe gerada no stderr uma vez e nunca mais. Até lá a única entrada é o testemunho de emergência abaixo.
login-emergency-summary = Acesso de emergência
login-emergency-detail = O testemunho partilhado continua a abrir uma sessão de administrador e existe para uma coisa só: voltar a entrar quando o ficheiro .acl se perdeu ou todos os administradores ficaram de fora. Não é uma conta: não tem preferências próprias, e o diário de alterações anota-o como emergency-token em vez de como pessoa. O seu uso é registado como aviso.
login-emergency-label = Testemunho de emergência
login-emergency-submit = Usar o testemunho de emergência
login-sign-in-prompt = Entre para chegar ao painel de administração.

## Administração

admin-title = Administração
admin-lede = A editar { $path } — { $total } elementos, { $files ->
        [one] um ficheiro anexado
       *[other] { $files } ficheiros anexados
    }, { $size } em disco. Cada alteração é escrita de uma vez; uma alteração recusada deixa o ficheiro intacto.
admin-entities = Elementos
admin-create = Criar
admin-new-kind = Novo: { $kind }
admin-operations = Operações
admin-validate = Verificar
admin-deduplicate = Juntar duplicados
admin-export = Exportar o arquivo
admin-accounts = Contas
admin-roles-note = Verificar, juntar duplicados, exportar, apagar e gerir contas é só do administrador. Um colaborador chega a todas as outras páginas daqui.
admin-dedup-confirm = Juntar duplicados funde fichas e reescreve o arquivo. Continuar?
admin-recent-changes = Alterações recentes
admin-recent-note = As últimas { $shown } de { $total ->
        [one] uma alteração registada
       *[other] { $total } alterações registadas
    }, de { $path }.
admin-sessions-open = { $n ->
        [one] Uma sessão aberta neste momento.
       *[other] { $n } sessões abertas neste momento.
    }
admin-no-changes-yet = Com esta aplicação ainda não foi mudado nada. Cada gravação daqui em diante fica anotada em { $path }.
admin-last-validation = Última verificação
admin-bundle-heavy = Este arquivo pesa { $size }. É carregado por inteiro ao arrancar e mantido em memória, por isso acima de cerca de { $warn } o sítio começa a custar memória a sério e os reinícios ficam lentos. Isso serve a um arquivo de família, não a uma mediateca: se os anexos crescerem sem limite, guarde-os num depósito de ficheiros e faça o arquivo apontar para eles.

admin-fields = Campos
admin-raw-json = JSON em bruto
admin-raw-json-help = O elemento inteiro, para que nada fique por editar: listas como os parceiros e os filhos de uma família, ou a história de fronteiras de um lugar, vivem precisamente aqui. Este é o documento de partida; os campos acima são depois escritos sobre os caminhos que lhes pertencem, por isso edite um valor num sítio ou no outro, não nos dois. Tem de ler-se como JSON, senão não se guarda nada.
admin-save = Guardar
admin-cancel = Cancelar
place-editor-title = Editar um lugar
place-add-detail = Completar este lugar
place-names = Nomes
place-name-primary = Principal
place-name-lang = Língua
place-name-value = Nome
place-names-hint = Uma linha por cada nome registado. Um local administrado por três impérios tem três nomes; o principal é o que se mostra em todo o lado.
place-where = Localização
place-type = Tipo
place-region = Região
place-country-current = País hoje
place-country-hint = ISO 3166-1 alfa-2, por exemplo PL, FR, DE.
place-country-history = História das fronteiras
place-history-country = Estado
place-history-from = De
place-history-until = Até
place-country-history-hint = Que Estado deteve este local e em que período. Importa em genealogia: um assento escrito em russo em 1880 e outro escrito em polaco em 1930 podem nomear a mesma aldeia.
place-coordinates = Coordenadas
place-lat = Latitude
place-lon = Longitude
place-precision = Precisão
place-identifiers = Identificadores
place-wikidata = Wikidata
place-geonames = GeoNames
place-used-by = { $name } é usado por { $n ->
        [one] outro registo
       *[other] outros { $n } registos
    }.
place-error-no-name = Um local precisa de pelo menos um nome.
place-error-coords-pair = Latitude e longitude andam juntas: indique as duas ou nenhuma.
place-error-coords-number = A latitude e a longitude têm de ser números.
place-error-coords-range = A latitude vai de -90 a 90 e a longitude de -180 a 180.
place-type-continent = continente
place-type-country = país
place-type-region = região
place-type-department = departamento
place-type-city = cidade
place-type-village = aldeia
place-type-district = bairro
place-type-street = rua
place-type-building = edifício
place-type-farm = quinta
place-type-island = ilha
place-type-historical = histórico
place-type-unknown = desconhecido
place-precision-exact = exata
place-precision-building = edifício
place-precision-street = rua
place-precision-city_center = centro da cidade
place-precision-region_center = centro da região
place-precision-country_center = centro do país
place-precision-approximate = aproximada

place-coordinates-hint = Escrever à mão é o caminho habitual. Muitos lugares registados sob uma administração anterior não aparecem numa pesquisa moderna.
place-geocode-search = Procurar este nome
place-geocode-hint = Envia o nome, a região e o país ao serviço de geocodificação, um lugar de cada vez. Nada é guardado até que guarde.
place-geocode-off = A procura de nomes está desligada. Precisa de um endereço de contacto pelo qual o serviço identifique esta instalação; inicie o servidor com --geocoder-contact para a ligar.
place-geocode-query = Procurou-se: { $q }
place-geocode-error = Não foi possível contactar o serviço de procura. Os campos de coordenadas acima continuam a funcionar.
place-geocode-none = Nada encontrado. Para uma aldeia registada sob administração russa, prussiana ou austríaca este é o resultado comum; introduza a posição à mão.
place-geocode-not-a-place = não é uma povoação
place-geocode-use = Usar este
place-geocode-attribution = Resultados do OpenStreetMap via Nominatim, sob a Open Database License.

place-paste = Colar uma posição
place-paste-placeholder = uma ligação de mapa, ou 52.0782795, 21.2508068
place-paste-read = Ler
place-paste-hint = Uma ligação do Google Maps ou do OpenStreetMap, um URI geo:, um par de números, ou graus-minutos-segundos como 52°04'41.8"N 21°15'02.9"E.
place-paste-read-ok = Lida para os campos acima. Verifique e depois guarde.
place-paste-unreadable = Essa não é uma posição que se possa ler aqui. Os campos acima continuam a aceitar um par de números.

place-map-hint = Clique no mapa para pôr o ponto, ou arraste o alfinete. O que vale são os campos acima.
place-map-clear = Retirar o ponto
place-open-in-map = Procurar este lugar no OpenStreetMap e colar aqui a ligação

person-tab-record = Ficha
person-tab-life = Vida
person-tab-media = Materiais
person-tab-tree = Árvore
person-tree-depth = { $n } gerações para cada lado. A árvore inteira está mais abaixo.
person-tree-alone = Esta ficha não nomeia pais, cônjuges nem filhos, por isso não há forma nenhuma a desenhar à sua volta.

record-no-evidence = A esta ficha não está anexado nada — nem fonte nem documento. É o estado comum de um ficheiro convertido, não um defeito seu: o GEDCOM leva os factos e deixa para trás o que os provava.
record-no-evidence-signed-out = Inicie sessão para anexar algo.
admin-delete = Apagar
admin-not-set = — por definir —
admin-edit = Editar
admin-page-of = Página { $page } de { $pages }
admin-previous = Anterior
admin-next = Seguinte
admin-saved = Guardado como versão { $version } — { $summary }
admin-not-saved = Não guardado
admin-created = Criado
admin-not-created = Não criado
admin-deleted = Apagado
admin-not-deleted = Não apagado — nada foi alterado
admin-what-changed = o que mudou
admin-field = Campo
admin-from = De
admin-to = Para
admin-version = versão { $version }

## Contas

accounts-title = Contas
accounts-lede = Guardadas em { $path }, com permissões 600, ao lado do arquivo e nunca dentro. Um arquivo copia-se, envia-se e publica-se; os resumos das palavras-passe a viajar lá dentro fariam de cada cópia da árvore uma cópia das credenciais.
accounts-existing = Existentes
accounts-username = Nome de utilizador
accounts-role = Papel
accounts-status = Estado
accounts-branch = Ramo
accounts-last-seen = Último acesso
accounts-change = Alterar
accounts-you = (você)
accounts-active = activa
accounts-disabled = desactivada
accounts-never = nunca
accounts-whole-tree = árvore inteira
accounts-roots = { $n ->
        [one] uma raiz
       *[other] { $n } raízes
    }
accounts-add = Adicionar uma conta
accounts-no-registration = De propósito não há inscrição por conta própria nem convites. Para um arquivo de família basta um administrador que conheça toda a gente, e isso retira por inteiro uma superfície de abuso em vez de ter de a defender.
accounts-password-hint = Deixe em branco e é gerada uma, mostrada uma só vez. Pelo menos { $min } caracteres se a definir você.
accounts-new-password-placeholder = nova palavra-passe (em branco = manter)
accounts-email = Correio electrónico
accounts-optional = (facultativo)
accounts-create = Criar a conta
accounts-role-viewer = leitura — lê as fichas públicas e as da família
accounts-role-contributor = colaborador — além disso cria, edita e envia ficheiros
accounts-role-admin = administrador — além disso gere contas, apaga e exporta
accounts-branch-hint = Limita o que esta conta pode editar a essas pessoas, aos seus descendentes e aos seus cônjuges.
accounts-branch-reading = Não limita o que pode ler: isso é governado pela visibilidade de cada ficha, e as duas coisas mantêm-se separadas de propósito.
accounts-branch-placeholder = um identificador de pessoa por linha
accounts-ids-in-bundle = Identificadores de pessoas nesta árvore
accounts-emergency-warning = Entrou com o testemunho de emergência. Concede direitos de administrador nesta sessão mas não é uma conta: não tem preferências próprias, e o diário anotará as suas alterações como emergency-token em vez de como pessoa. Crie abaixo uma conta a sério e entre com ela.
accounts-created-with-password = Criada { $username }. A palavra-passe é { $password } — é mostrada uma só vez e guardada apenas como resumo Argon2id, por isso passe-a agora.
accounts-created = Criada { $username }.
accounts-updated = Actualizada { $username }. Qualquer sessão que tivesse aberta foi terminada.
accounts-username-taken = Esse nome de utilizador já está ocupado.
accounts-pick-role = Escolha um papel.
accounts-no-such = Não existe essa conta.
accounts-last-admin = É o único administrador activo. Promova antes outra pessoa: uma instalação sem administrador só se recupera editando o ficheiro .acl ou usando o testemunho de emergência.
accounts-not-saved = Não guardado: { $error }

## Conflitos

conflict-title = Outra pessoa alterou isto primeiro
conflict-lede = { $who } guardou uma alteração a este elemento ({ $kind }) às { $when }, depois de o ter aberto. A sua edição não foi guardada e nada foi substituído.
conflict-no-merge = Aqui nada se junta automaticamente. Fundir as edições de duas pessoas produz uma ficha que nenhuma delas escolheu, e em genealogia dois redactores em desacordo sobre uma data costumam estar a ler fontes diferentes — e isso é uma pergunta para uma pessoa, não para um programa. Compare as duas abaixo e decida.
conflict-versions = Partiu da versão { $expected }; a ficha está agora na versão { $current }.
conflict-both-changed = Alteraram isto os dois
conflict-both-changed-detail = Estes campos foram editados por ambos. O que quer que guarde substituirá o que { $who } lá pôs:
conflict-different-fields = Alteraram campos diferentes, por isso nada do trabalho de { $who } está em causa — mas voltar a aplicar escreve na mesma o seu elemento inteiro por cima do da outra pessoa. Verifique as duas colunas antes de guardar.
conflict-field-by-field = Campo a campo
conflict-theirs = Para o que { $who } o mudou
conflict-yours = Para o que você o mudou
conflict-unchanged-by-you = não alterado por si
conflict-unchanged-by-them = não alterado por eles
conflict-nothing-differs = Nenhuma das versões difere daquela de que partiu em nenhum campo que esta página mostre. O número de versão avançou, portanto alguém guardou a ficha sem mudar nada do que ela contém.
conflict-what-now = E agora
conflict-reapply = Voltar a aplicar a sua versão por cima da deles
conflict-reapply-hint = Esta é a sua edição, transportada para a versão { $version }. Corrija-a aqui para manter o que quiser do trabalho de { $who } e depois guarde. A versão deles está abaixo, para copiar.
conflict-save-over = Guardar esta por cima da deles
conflict-discard = Descartar a minha e recomeçar
conflict-their-version = A versão de { $who }, tal como está agora
conflict-history-of = Histórico deste elemento ({ $kind })

## Importação

convert-title = Importar um ficheiro familiar
convert-submit = Importar
convert-result-title = Relatório de importação
convert-download = Transferir o arquivo

## Datas

date-unknown = Data desconhecida
date-not-recorded = Não registada
date-circa = cerca de { $date }
date-between = entre { $from } e { $to }
date-before = antes de { $date }
date-after = depois de { $date }
date-preserved = registada como «{ $text }»
date-day-month-year = { $day } de { $month ->
        [1] janeiro
        [2] fevereiro
        [3] março
        [4] abril
        [5] maio
        [6] junho
        [7] julho
        [8] agosto
        [9] setembro
        [10] outubro
        [11] novembro
        [12] dezembro
        *[other] { $month }
    } de { $year }
date-month-year = { $month ->
        [1] janeiro
        [2] fevereiro
        [3] março
        [4] abril
        [5] maio
        [6] junho
        [7] julho
        [8] agosto
        [9] setembro
        [10] outubro
        [11] novembro
        [12] dezembro
        *[other] { $month }
    } de { $year }
date-decade = os anos { $decade }
date-century = o século { $century }
date-quarter-century = o { $quarter ->
        [1] primeiro
        [2] segundo
        [3] terceiro
       *[other] quarto
    } quartel do século { $century }

## Mais páginas de erro

error-back-to-start = Voltar ao início
error-payload-missing-title = Não há tal ficheiro
error-payload-missing-detail = O conteúdo desse documento não está na cache.
error-payload-unopenable-detail = O conteúdo desse documento não se conseguiu abrir.
error-no-such-document-detail = Aqui não há nenhum documento com esse identificador.
error-bad-preference-title = Não é uma das opções
error-bad-preference-detail = Não é uma língua nem um aspecto que este sítio ofereça. Nada foi alterado.
error-unknown-kind-title = Espécie desconhecida
error-unknown-kind-detail = «{ $kind }» não é uma espécie de ficha. Este arquivo contém: { $kinds }.
error-io-title = Não foi possível guardar
error-io-detail = { $error }. No disco nada foi alterado.
error-upload-too-large = Esse ficheiro passa o limite de { $mb } MB. Nada foi guardado e o arquivo fica igual.
error-upload-refused = O documento foi recusado: { $reason }. O arquivo fica igual.
error-back-to-person = Voltar à ficha
error-no-such-person-to-attach = Aqui não há nenhuma pessoa com esse identificador, portanto não há a que anexar um documento.
error-upload-title = Esse envio não foi guardado
error-download-expired-title = Essa transferência expirou
error-download-expired-detail = Uma importação guarda-se quinze minutos e depois é descartada. Importe o ficheiro outra vez.
error-upload-none = Não foi enviado nenhum ficheiro. Escolha primeiro um.
error-upload-unsupported = Esse tipo de ficheiro o arquivo não guarda. Aceitam-se imagens, PDF, texto simples, áudio e vídeo; o tipo lê-se dos próprios bytes do ficheiro, por isso mudar o nome a um executável não serve. O SVG é recusado sem mais, porque um SVG pode levar um script.
error-export-unreadable-title = Não foi possível ler o arquivo exportado
error-export-unreadable-detail = { $error }

## Página da árvore, continuação

tree-title-suffix = árvore
tree-back-to-focused = Voltar à vista à volta de uma pessoa
tree-show-all = Mostrar as { $n }
tree-width-notice = Esta vista tem { $width } pixéis de largura: num ecrã de 1500 pixéis são { $screens ->
        [one] um ecrã
       *[other] { $screens } ecrãs
    } de deslocamento na horizontal.
tree-confidence-label = Certeza:
tree-band-certain = certo
tree-band-high = alta
tree-band-medium = média
tree-band-low = hipotético
tree-counts = { $drawn } de { $total } pessoas · { $generations ->
        [one] uma geração
       *[other] { $generations } gerações
    }
tree-unplaced-count = { $n } sem lugar
tree-contradicts-title = Esta árvore contradiz-se.
tree-contradicts-detail = Nenhuma disposição de linhas pode satisfazer isso, por isso o parentesco abaixo ficou fora da numeração das gerações e alguma linha pode estar desenhada no sítio errado. Corrija aquela das duas fichas que está errada.
tree-contradicts-pair = Registados ao mesmo tempo como casal e como progenitor e filho:
tree-contradicts-more = { $n ->
        [one] Outra contradição não está listada.
       *[other] Outras { $n } contradições não estão listadas.
    }
tree-no-people = Nesta árvore ainda não há ninguém.
tree-no-people-cta = Importe um ficheiro familiar, ou acrescente a primeira pessoa.
tree-nobody-selected = Para essa selecção não há ninguém para desenhar.
tree-nobody-selected-cta = Comece pela vista por omissão.
tree-edge-union = Uma união registada
tree-edge-parentage = Uma filiação registada

## Página inicial

home-empty = Ainda não há nada registado. Importe um ficheiro familiar para trazer uma árvore já existente, ou acrescente à mão a primeira pessoa.
home-count = { $total ->
        [one] Uma ficha
       *[other] { $total } fichas
    }, num único ficheiro que é da família.
home-browse = Percorrer a árvore
home-convert = Importar um ficheiro familiar
home-unnamed-family = Esta árvore de família
home-in-this-tree = O que a família registou até agora
home-showcase-title = Onde esta árvore já diz mais do que nomes e datas
home-showcase-example = Ver um exemplo →
home-nothing-title = Ainda não há nada para mostrar.
home-nothing-detail = Importe um ficheiro familiar para trazer uma árvore já existente, ou comece do zero e acrescente você mesmo a primeira pessoa.

## Cartões de mostra

showcase-links-title = { $n ->
        [one] Uma relação fora da família
       *[other] { $n } relações fora da família
    }
showcase-links-detail = Padrinhos, patrões, testemunhas e mestres, cada um com as suas próprias datas, a sua fonte e o seu grau de certeza.
showcase-occupations-title = { $n ->
        [one] Uma ocupação com um princípio e um fim
       *[other] { $n } ocupações com um princípio e um fim
    }
showcase-occupations-detail = «Professora primária, 1948-1978» conserva a sua duração e é desenhada como uma barra ao longo dos anos, não como uma única linha datada.
showcase-uncertain-title = { $n ->
        [one] Uma data deixada tão imprecisa como foi dada
       *[other] { $n } datas deixadas tão imprecisas como foram dadas
    }
showcase-uncertain-detail = Cerca de, antes, depois e entre continuam a ser quatro afirmações diferentes. Uma data que a fonte não soube fixar nunca é mostrada como se a tivesse fixado.
showcase-preserved-title = { $n ->
        [one] Uma data guardada nas palavras em que foi escrita
       *[other] { $n } datas guardadas nas palavras em que foram escritas
    }
showcase-preserved-detail = Uma formulação que ninguém soube ler como data fica exactamente como está escrita, em vez de ser descartada em silêncio.
showcase-sources-title = { $n ->
        [one] Uma fonte com a sua fiabilidade registada
       *[other] { $n } fontes com a sua fiabilidade registada
    }
showcase-sources-detail = { $primary ->
        [one] Uma fonte primária.
       *[other] { $primary } primárias.
    } Cada facto mostra em que prova se apoia e que força tem essa prova.
showcase-places-title = { $n ->
        [one] Um lugar cujas fronteiras se moveram
       *[other] { $n } lugares cujas fronteiras se moveram
    }
showcase-places-detail = Uma cidade pode pertencer a estados diferentes em épocas diferentes, e a ficha diz qual valia quando.

## Detalhes da ficha

record-also-recorded-as = registado também como
record-borders-moved = Fronteiras movidas:
record-display-name = nome mostrado
record-read-as = lido como
record-note = Nota
record-living-yes = vivo
record-deceased = falecido/a
record-centre-tree-here = Centrar a árvore aqui
record-centre-tree-title = Mover a árvore para a centrar nesta pessoa
record-open-full-page = Abrir a página inteira ↗
record-open-full-title = Abrir a página independente que se pode partilhar
record-edit = Editar
panel-empty = Escolha uma ficha para ver aqui o documento completo dessa pessoa.
person-see-in-tree = Ver esta pessoa na árvore
person-visibility-inline = visibilidade:
person-age-at-death = morreu aos { $n }
person-age-now = { $n } anos
person-born-in = nascido em { $place }
person-died-in = morreu em { $place }
person-children-count = { $n ->
        [one] um filho
       *[other] { $n } filhos
    }
person-generations-below = { $n ->
        [one] uma geração abaixo
       *[other] { $n } gerações abaixo
    }
person-portrait-of = Fotografia de { $name }
person-no-portrait = Sem fotografia registada

## Resultados das operações

result-diagnostics = Avisos
result-diagnostics-note = Todos os avisos devolvidos pela biblioteca, incluindo as advertências que não travaram a operação. Nenhum é filtrado.
result-no-diagnostics = A biblioteca não devolveu avisos.
result-continue = Continuar
result-dashboard = Painel
person-sections-label = Secções desta página

## Vocabulary the structured editors offer

name-part-nasab = nasab (linhagem)
name-part-laqab = laqab (epíteto)
name-part-kunya = kunya (teknónimo)
name-part-nisbah = nisbah (origem)
name-part-alias = alcunha
name-part-religious_name = nome religioso
name-part-pen_name = pseudónimo
name-type-pen_name = pseudónimo
gender-U = Sem registo

## Secções da ficha, detalhes

record-notes-title = A reter sobre esta ficha:
record-name = Nome
record-type = Tipo
record-cause = Causa:
record-as = como
record-partner-not-recorded = Parceiro não registado
record-union-from = Desde
record-union-at = em
record-union-until = até
record-occupation-from = desde
record-occupation-until = até
record-source-reliability = Fiabilidade
record-source-supports = Sustenta
record-photographs = Fotografias
record-documents = Documentos
record-file = Ficheiro
record-status = Estado
record-size = Tamanho
record-absent-document = Nomeado por esta pessoa mas não guardado aqui.
record-no-file = sem ficheiro
record-attach-document = Anexar um documento
record-doc-photo = foto
record-doc-certificate = assento
record-doc-letter = carta
record-doc-record = registo
record-doc-newspaper = jornal
record-doc-other = outro
record-upload = Enviar
record-upload-help = Até { $mb } MB por ficheiro. Os anexos ficam ao lado da árvore e são reescritos no arquivo ao exportar, por isso uma fotografia viaja com a família a que pertence. A espécie de ficheiro lê-se do seu próprio conteúdo e não do nome: aceitam-se imagens, PDF, texto simples, áudio e vídeo. O SVG é recusado, porque um SVG pode levar um script.
record-upload-help-short = Até { $mb } MB. O SVG é recusado.
record-verbatim-note = Guardado tal como a ficha o dava, porque nenhum conversor o soube interpretar.
record-file-to-attach = Ficheiro a anexar
record-document-type = Tipo de documento
record-caption = Legenda
record-caption-placeholder = Legenda (facultativa)
record-history-entry-meta = — { $at }
record-history-entry-version = , { $version }

## Espécies de elemento

kind-person = pessoa
kind-family = família
kind-event = facto
kind-link = vínculo
kind-occupation = ocupação
kind-source = fonte
kind-place = lugar
kind-document = documento

kind-person-plural = { $n ->
        [one] pessoa
       *[other] pessoas
    }
kind-family-plural = { $n ->
        [one] família
       *[other] famílias
    }
kind-event-plural = { $n ->
        [one] facto
       *[other] factos
    }
kind-link-plural = { $n ->
        [one] vínculo
       *[other] vínculos
    }
kind-occupation-plural = { $n ->
        [one] ocupação
       *[other] ocupações
    }
kind-source-plural = { $n ->
        [one] fonte
       *[other] fontes
    }
kind-place-plural = { $n ->
        [one] lugar
       *[other] lugares
    }
kind-document-plural = { $n ->
        [one] documento
       *[other] documentos
    }

## Listagens

list-matching = { $total ->
        [one] Uma correspondência
       *[other] { $total } correspondências
    }, { $per_page } por página.
list-filter-placeholder = Filtrar por nome ou identificador
list-filter = Filtrar
list-clear = Limpar
list-summary = Descrição
list-id = Identificador
list-actions = Acções
list-nothing = Aqui não há nada.
list-nothing-matching = Aqui não há nada que corresponda a «{ $q }».
list-delete-confirm = Apagar este elemento ({ $kind })? Escolha o que acontece aos elementos que o citam:
list-policy-reject = Recusar
list-policy-reject-detail = — recusar se alguma coisa ainda o citar. Nada se perde.
list-policy-cascade = Em cascata
list-policy-cascade-detail = — apagá-lo e retirar mesmo todas as citações a ele.
list-policy-orphan = Deixar órfãos
list-policy-orphan-detail = — apagá-lo mas manter as fichas que o citam, com a citação esvaziada.

## Grau de detalhe

completeness-dates-title = As datas segundo a forma que realmente têm
completeness-no-dates = Ainda não há datas registadas.
completeness-dates-note = Uma data que alguém soube fixar ao dia e outra que alguém só soube situar numa década são duas afirmações diferentes, e ambas ficam como foram dadas. O texto que não se conseguiu ler como data guarda-se palavra por palavra em vez de ser descartado.
completeness-shape-exact = exacta
completeness-shape-exact-note = um dia de calendário inteiro
completeness-shape-approximate = aproximada
completeness-shape-approximate-note = cerca de, ou só um ano ou uma década
completeness-shape-ranged = delimitada
completeness-shape-ranged-note = antes, depois ou entre
completeness-shape-preserved = à letra
completeness-shape-preserved-note = texto não interpretável, guardado tal e qual
completeness-shape-unknown = desconhecida
completeness-shape-unknown-note = registada como não sabida

## Página de importação

convert-page-title = Importar um ficheiro familiar
convert-lede = Traga uma árvore já existente a partir de um ficheiro GEDCOM, a exportação que a maior parte dos programas de genealogia produz. Aqui não se guarda nada, e a árvore que este sítio já mostra fica exactamente como estava.
convert-file-label = Ficheiro familiar (.ged)
convert-file-hint = Até { $mb } MB. Uma árvore de 767 pessoas pesa cerca de 320 KB.
convert-confidence-label = Que certeza têm estes factos, para começar
convert-confidence-hint = O ficheiro a importar não diz que segurança alguém tinha, por isso cada facto precisa de um ponto de partida. Ponha-o baixo para uma árvore reunida à pressa, mais alto para uma trabalhada sobre documentos. A leitura honesta deste número é «importado, e desde então ninguém verificou»: poderá subir ou baixar cada facto depois, um a um.
convert-lang-label = Língua dos nomes de lugar
convert-lang-hint = Uma etiqueta como en, fr ou pt.

## Relatório de importação

convert-failed = A importação não foi por diante
convert-try-another = Tentar outro ficheiro
convert-converted = Importado { $filename }
convert-result-lede = { $total ->
        [one] Uma ficha
       *[other] { $total } fichas
    }, { $size } KB. Entrou tudo com uma certeza de { $confidence }, com os nomes de lugar lidos como { $lang }. A árvore que este sítio mostra não foi tocada.
convert-produced = O que passou
convert-skipped-title = { $n ->
        [one] Uma entrada que não se conseguiu ler
       *[other] { $n } entradas que não se conseguiram ler
    }
convert-skipped-note = Estas entradas não continham nada que se pudesse trazer.
convert-other-diagnostics = { $n ->
        [one] Outra coisa que convém saber
       *[other] Outras { $n } coisas que convém saber
    }
convert-clean = Nada ficou para trás — todas as entradas do ficheiro passaram.
convert-download-title = Transferência
convert-download-named = Transferir { $name }
convert-download-note = Guardado aqui quinze minutos e depois descartado, por isso transfira-o agora.
convert-another = Importar outro ficheiro
admin-history-on = a
admin-history-meta = — { $kind }, { $at }
admin-validation-counts = { $errors ->
        [one] Um erro
       *[other] { $errors } erros
    }, { $warnings ->
        [one] uma advertência
       *[other] { $warnings } advertências
    }, { $infos ->
        [one] uma nota
       *[other] { $infos } notas
    }.
admin-warnings-never-block = As advertências nunca travam nada: são informação, não uma barreira.
admin-validator-clean = A verificação não comunicou nada.
record-occupations-help-undated = Uma ocupação regista-se com princípio e fim, de modo que várias se comparem sobre uma mesma linha de tempo. Este arquivo tem os nomes dos ofícios mas não as datas — coisa normal depois de uma importação, porque a maior parte dos ficheiros familiares não tem onde as guardar —, por isso ainda não há escala que desenhar.
record-occupations-help-axis = Uma ocupação é um estado com duração, não um facto numa só data. Todos os troços partilham um eixo, { $from }–{ $to }.
admin-value-not-set = por definir
admin-validation-report = Relatório de verificação
admin-dedup-complete = Junção de duplicados concluída
admin-dedup-refused = Junção de duplicados recusada
record-birth-order = ordem de nascimento
record-start-not-recorded = princípio não registado
record-end-not-recorded = fim não registado
record-document-no-file = O documento está registado aqui, mas o ficheiro em si não está
panel-selected-person = Pessoa escolhida

## Faixas de geração

tree-band-generation = Geração { $g }
tree-band-people = { $n ->
        [one] uma pessoa
       *[other] { $n } pessoas
    }
tree-band-unplaced = Sem lugar
tree-band-unplaced-note = { $n ->
        [one] uma pessoa sem família — mostrada em vez de omitida
       *[other] { $n } pessoas sem família — mostradas em vez de omitidas
    }

## Vocabulário controlado

gender-M = Masculino
gender-F = Feminino
gender-NB = Não binário
gender-unrecorded = Não registado

name-part-given_name = nome próprio
name-part-family_name = apelido
name-part-patronymic = patronímico
name-part-matronymic = matronímico
name-part-middle_name = nome do meio
name-part-nickname = alcunha
name-part-prefix = prefixo
name-part-suffix = sufixo
name-part-particle = partícula
name-part-part = elemento

name-type-primary = principal
name-type-other = outro
name-type-alias = de uso
name-type-birth = de nascimento
name-type-married = de casada
name-type-religious = religioso
name-type-transliteration = transliteração
name-type-nickname = alcunha

## Anotações sobre a ficha

note-links = { $n ->
        [one] uma relação fora da família, com datas e fontes próprias
       *[other] { $n } relações fora da família, com datas e fontes próprias
    }
note-occupations = { $n ->
        [one] um ofício registado com princípio e fim
       *[other] { $n } ofícios registados com princípio e fim
    }
note-birth-imprecise = uma data de nascimento que a fonte não soube fixar, mostrada tal como está registada
note-death-imprecise = uma data de óbito que a fonte não soube fixar, mostrada tal como está registada
note-names = { $n ->
        [one] um nome registado
       *[other] { $n } nomes registados
    }
note-transliteration = um nome na sua própria escrita ao lado da transliteração latina
note-witnessed = { $n ->
        [one] um facto de que foi testemunha e não protagonista
       *[other] { $n } factos de que foi testemunha e não protagonista
    }

visibility-public = pública
visibility-members = familiares
visibility-contributors = colaboradores
visibility-private = reservada

## Descrições de linha nas listagens de administração

family-label-couple = { $children ->
        [0] { $a } e { $b }
        [one] { $a } e { $b } — um filho
       *[other] { $a } e { $b } — { $children } filhos
    }
family-label-half = { $children ->
        [0] { $a } e { $unknown }
        [one] { $a } e { $unknown } — um filho
       *[other] { $a } e { $unknown } — { $children } filhos
    }
family-label-children = { $others ->
        [0] { $first } — pais não registados
        [one] { $first } e um irmão — pais não registados
       *[other] { $first } e { $others } irmãos — pais não registados
    }
family-label-empty = Família sem ninguém registado

event-label = { $category } — { $who }, { $date }
event-label-nobody = { $category } — { $date }
event-two-people = { $a } e { $b }
event-more-people = { $a } e { $b } e mais { $others ->
        [one] um
       *[other] { $others }
    }

link-label = { $label }: { $from } → { $to }
occupation-label = { $who } — { $title }
source-label = { $title } — { $reliability }
source-label-plain = { $title }
document-label = { $filename } — { $type }
document-label-untitled = { $type } sem título
list-unnamed = { $kind } sem nome

## Vocabulários da especificação nas listagens

event-category-birth = Nascimento
event-category-death = Óbito
event-category-marriage = Casamento
event-category-divorce = Divórcio
event-category-baptism = Baptismo
event-category-burial = Enterro
event-category-immigration = Imigração
event-category-emigration = Emigração
event-category-census = Recenseamento
event-category-residence = Residência
event-category-military = Serviço militar
event-category-education = Estudos
event-category-other = Facto

reliability-primary = fonte primária
reliability-secondary = fonte secundária
reliability-tertiary = fonte terciária
reliability-recollection = testemunho oral
reliability-derivative = obra derivada
reliability-authored = obra de autor
reliability-oral = tradição oral
reliability-unknown = fiabilidade desconhecida

document-type-photo = fotografia
document-type-certificate = assento
document-type-letter = carta
document-type-record = registo de arquivo
document-type-newspaper = recorte de jornal
document-type-other = documento

## Onde esta ficha poderia dizer mais

completeness-title = Onde esta árvore poderia dizer mais
completeness-intro = O que está registado e o que continua em branco.
completeness-import-title = O que a importação trouxe
completeness-import-intro = Contado sobre o ficheiro que acabou de enviar. Uma linha em branco é algo que o ficheiro de origem não registava, não algo que a importação tenha perdido.

completeness-headline-full = Cada espécie de pormenor abaixo está registada nalgum sítio desta árvore.
completeness-headline-empty = { $total ->
        [one] A única espécie de pormenor abaixo ainda não está registada em lado nenhum.
       *[other] Nenhuma das { $total } espécies de pormenor abaixo está ainda registada.
    } Cada uma é um sítio onde a ficha poderia dizer mais.
completeness-headline-partial = { $carried ->
        [one] Uma espécie de pormenor abaixo está registada
       *[other] { $carried } espécies de pormenor abaixo estão registadas
    }; { $empty ->
        [one] uma continua em branco
       *[other] { $empty } continuam em branco
    }.

completeness-metric-confidence = Que certeza tem cada facto
completeness-metric-confidence-none = Nenhum dos { $slots } factos aqui diz que certeza tem. Uma data lida num assento e outra adivinhada parecem-se, até deixarem de se parecer.
completeness-metric-confidence-uniform = { $with } de { $slots } factos levam uma pontuação, e todas são o mesmo número ({ $modal }). É o que uma importação em bloco deixa atrás de si: um valor de recurso a que ninguém voltou. Nenhum foi ainda julgado um a um.
completeness-metric-confidence-some = { $with } de { $slots } factos levam uma pontuação. { $modal_count } partilham um mesmo valor ({ $modal }); { $assessed } afastam-se dele e foram por isso vistos um de cada vez.
completeness-metric-confidence-many = { $with } de { $slots } factos levam uma pontuação, e { $assessed } deles afastam-se do valor mais frequente ({ $modal }), ao longo de { $distinct } níveis distintos. Esta árvore regista uma incerteza real e variada.

completeness-metric-parentage = Que certeza tem cada vínculo pai-filho
completeness-metric-parentage-none = Nenhuma filiação aqui diz que certeza tem. As adopções, as linhas contestadas e as reconstituições a partir de uma só menção são justamente os sítios onde uma família precisa de registar a dúvida — e a árvore desenha um vínculo menos seguro com uma linha mais pálida.
completeness-metric-parentage-some = { $n ->
        [one] Uma filiação leva a sua própria pontuação
       *[other] { $n } filiações levam a sua própria pontuação
    }, por isso uma linha hipotética é visivelmente mais fraca do que uma documentada.

completeness-metric-links = Relações para além do sangue e do casamento
completeness-metric-links-none = Padrinhos, patrões, testemunhas, mestres, tutores. Ainda não há nenhuma registada. Cada uma pode levar as suas próprias datas, a sua fonte e o seu grau de certeza.
completeness-metric-links-some = { $n ->
        [one] Uma registada, com as suas próprias datas, fonte e o seu grau de certeza.
       *[other] { $n } registadas, cada uma com as suas próprias datas, fonte e o seu grau de certeza.
    }

completeness-metric-occupations = Ofícios registados com princípio e fim
completeness-metric-occupations-none = Não há ocupações registadas. Um ofício exercido durante trinta anos diz mais de uma vida do que uma única entrada datada.
completeness-metric-occupations-undated = { $total ->
        [one] Está registada uma ocupação, sem datas
       *[other] Estão registadas { $total } ocupações, sem datas
    }. Acrescente um princípio e um fim e poderão comparar-se lado a lado sobre uma mesma linha de tempo.
completeness-metric-occupations-some = { $span } de { $total } têm princípio ou fim, portanto podem comparar-se lado a lado sobre uma mesma linha de tempo.

completeness-metric-sources = Fontes com a fiabilidade avaliada
completeness-metric-sources-none = Não há fontes registadas. Dizer de onde veio um facto é o que permite a um parente verificá-lo mais tarde — ou discordar e dizer porquê.
completeness-metric-sources-some = { $graded } de { $total } dizem que força têm, por isso uma afirmação que se apoia num assento de nascimento não é visivelmente o mesmo que uma que se apoia numa recordação.

completeness-what-is-recorded = O que a ficha pode dizer
completeness-in-this-tree = Nesta árvore
completeness-not-yet = ainda não registado

## Papéis de um participante num facto

role-spouse = cônjuge
role-spouse_1 = primeiro cônjuge
role-spouse_2 = segundo cônjuge
role-subject = pessoa do registo
role-participant = participante
role-witness = testemunha
role-officiant = celebrante
role-informant = declarante
role-godparent = padrinho ou madrinha

phys-no-source = sem fonte
phys-col-date = Quando
phys-col-source = Fonte
phys-col-confidence = Confiança
phys-col-note = Nota
phys-field-height-cm = Altura
phys-field-weight-kg = Peso
phys-field-eye-colour = Cor dos olhos
phys-field-hair-colour = Cor do cabelo
phys-field-build = Compleição
phys-field-handedness = Lateralidade
phys-field-features = Sinais particulares
phys-field-military = Serviço militar
phys-field-languages = Línguas faladas
phys-field-blood-group = Grupo sanguíneo
phys-field-conditions = Doenças conhecidas
phys-field-operations = Operações e ferimentos
phys-field-cause-of-death = Causa da morte
phys-field-religion = Religião ou filiação
phys-field-health-notes = Notas
admin-export-health-note = A exportação simples deixa de fora todas as categorias sensíveis — saúde e crenças, dados biométricos, dados genómicos e registo criminal — e também o perfil comportamental de qualquer pessoa viva, pelo que um ficheiro enviado a um familiar não contém nenhuma delas. Assinale o que um ficheiro concreto deve conter; o próprio arquivo regista que categorias foram omitidas.
avatar-picker-title = Escolher uma imagem
avatar-choose-link = Escolher imagem
avatar-choose = Que imagem representa esta pessoa
avatar-mode-auto = Deixar o programa escolher
avatar-mode-auto-note = O primeiro retrato ou, na falta dele, a primeira imagem ligada a esta ficha.
avatar-mode-none = Mostrar as iniciais
avatar-mode-none-note = Para uma ficha cujas imagens são documentos e não rostos.
avatar-focal-hint = Clique numa imagem para a escolher e clique de novo na parte que deve ficar no enquadramento — um avatar é quadrado e a maioria das digitalizações não é.
avatar-no-images = Ainda não há imagens ligadas a esta ficha.
avatar-upload-title = Enviar uma imagem e usá-la
avatar-upload-button = Enviar e usar como imagem
avatar-not-available-title = Essa imagem não está disponível
avatar-not-available-detail = O ficheiro escolhido não está ligado a esta pessoa, ou não o pode ler.

record-history-withheld = não lhe é mostrado

## A death nobody wrote down
#
# GEDCOM cannot say "died, date unknown", so a converter marks those
# people living. These three say that the page worked it out rather than
# read it, which is a distinction this product does not blur.

record-life-status = Estado
record-presumed-deceased = óbito presumido
record-presumed-short = presumido
record-presumed-why = Não há óbito registado e o nascimento foi há mais de { $years } anos, por isso este registo não pode estar certo. O arquivo não é alterado: isto é o que a página infere, não o que a fonte diz.

## A figura gerada ao lado de um registo
#
# Não é um retrato. Todas as mensagens que se seguem existem para que isso
# fique sem ambiguidade.

silhouette-label = Idade e altura registadas, não uma aparência
silhouette-not-a-likeness = Não é um retrato: nada nela vem de uma fotografia.
silhouette-proportions-infant = Proporções de um bebé, cerca de quatro alturas de cabeça.
silhouette-proportions-child = Proporções de uma criança pequena, cerca de seis alturas de cabeça.
silhouette-proportions-adolescent = Proporções de um adolescente, cerca de sete alturas de cabeça.
silhouette-proportions-adult = Proporções de um adulto, cerca de sete alturas de cabeça e meia.
silhouette-proportions-elderly = Proporções de um adulto, que deixam de mudar por volta dos vinte anos: a figura não distingue uma pessoa idosa de uma adulta mais nova, e as datas acima distinguem.
silhouette-to-scale = Desenhada à escala, contra uma linha de referência a { $ref } cm.
silhouette-no-height = Não há altura registada, por isso a figura é desenhada num tamanho nominal para o seu escalão etário e não à escala.
silhouette-several-heights = Está registada mais do que uma altura; a figura desenha a mais recente.

## The identity editor

identity-editor-title = Nomes e identidade
identity-primary-name = O nome mostrado em toda a parte
identity-primary-help = O que a ficha da árvore, o título e todas as listas usam. Os outros nomes abaixo são os que uma fonte usou noutra altura.
identity-display = Nome
identity-display-latin = Em alfabeto latino
identity-culture = Idioma
identity-direction = Direcção
identity-direction-ltr = da esquerda para a direita
identity-direction-rtl = da direita para a esquerda
identity-direction-auto = a partir do texto
identity-components = Partes do nome
identity-components-help = Que parte é o nome próprio e qual o apelido, pela ordem em que se escrevem. Um registo sem partes mostra-se na mesma: as partes são aquilo que uma pesquisa consegue encontrar.
identity-part = Parte
identity-value = Texto
identity-other-names = Outros nomes
identity-other-help = Um apelido de casada, um nome religioso, um nome usado por um registo posterior. Cada um traz quando esteve em uso e que fonte o diz.
identity-name-type = Tipo de nome
identity-valid-from = Em uso desde
identity-valid-until = Em uso até
identity-about = Sobre a pessoa
identity-living-help = Esta é a marca que a fonte pôs. A página presume à parte um óbito quando o nascimento é demasiado antigo, e essa presunção nunca altera esta caixa nem o arquivo.
identity-error-no-display = Um registo precisa de um nome pelo qual ser mostrado. Nada foi guardado.
editor-blank-to-remove = Limpe o nome para remover esta entrada.
# A BCP 47 tag, shown as an example in the language field.
identity-culture-placeholder = pt
identity-edit-link = Editar nomes e identidade

## Union types, statuses and date precision, said out loud

union-type-marriage = casamento
union-type-civil_union = união civil
union-type-cohabitation = união de facto
union-type-religious_only = união religiosa
union-type-polygamous = polígamo
union-type-unknown = sem registo
union-status-active = em curso
union-status-ended_by_death = terminada por óbito
union-status-ended_by_divorce = terminada por divórcio
union-status-ended_by_separation = terminada por separação
union-status-annulled = anulada
union-status-unknown = sem registo
union-status-ended = terminada
union-status-ended-by = terminada por { $reason }
union-reason-death_of_spouse = o óbito de um cônjuge
precision-exact = ao dia
precision-year = ao ano
precision-month = ao mês
precision-decade = à década
precision-century = ao século
precision-unknown = desconhecida
record-precision = Precisão
record-approximate = Aproximado
record-place = Lugar

## The relationships editor

family-editor-title = Família e relações
family-unions = Uniões
family-no-unions = Não há nenhuma união registada para esta pessoa.
family-union-legend = União { $n }
family-writes-family = Guardar altera o registo de família #{ $id }, partilhado pelas duas pessoas. A página da outra pessoa muda também.
family-partners = Parceiros
family-partner = Parceiro
family-role = Papel
family-children = Filhos
family-children-help = A ordem de nascimento é a afirmação do próprio registo. Deixada em branco não afirma nada: um número tirado da posição da linha seria um facto que ninguém escreveu.
family-child = Filho
family-birth-order = Ordem de nascimento
family-the-union = A união em si
family-type = Tipo de união
family-status = Estado
family-started = Início
family-ended = Fim
family-leave = Retirar esta pessoa desta união
family-open-entity = Abrir o registo de família
family-new-union = Uma nova união
family-new-union-help = Isto cria um novo registo de família com esta pessoa. O parceiro é opcional: um progenitor que o registo nomeia sem parceiro nenhum é uma união de um.
family-create-union = Criar a união
family-parents = Pais
family-no-parents = Esta pessoa não está registada como filho de nenhuma família.
family-child-of = Filho desta família
family-detach-child = Retirar esta pessoa desta família
family-attach-parents = Ligar a pais
family-attach-help = Escolha a família de que esta pessoa é filho. Acrescenta-a a esse registo de família, por isso aparece também nas páginas dos pais.
family-the-family = A família
family-attach = Ligar
family-error-last-partner = Uma união precisa de pelo menos uma pessoa. Elimine antes o registo de família, que pergunta o que fazer a tudo o que lhe faz referência.
family-error-no-family = Não foi escolhida nenhuma família. Nada foi guardado.
family-error-already-child = Esta pessoa já é filho dessa família.
pick-error-empty = Não foi indicada nenhuma pessoa. Nada foi guardado.
pick-error-not-found = Não há ninguém com esse nome neste arquivo. Nada foi guardado.
pick-error-ambiguous = Mais do que uma pessoa corresponde. Escolha uma da lista para que o registo diga qual. Nada foi guardado.

## Links and occupations

links-editor-title = Ligações
links-editor-help = As relações que não são de família: um padrinho, um empregador, uma testemunha, um regimento. Cada uma é um registo próprio que nomeia duas pessoas, por isso editá-la aqui muda também o outro registo.
links-none = Não há nenhuma ligação registada para esta pessoa.
links-new = Uma nova ligação
links-create = Criar a ligação
links-remove = Remover esta ligação
links-other-end = A outra ponta
links-label = O que é
links-label-reverse = No sentido inverso
links-category = Categoria
links-bidirectional = Lê-se igual nos dois sentidos
links-from = Desde
links-until = Até
links-reversed = Esta ligação foi criada a partir do outro registo. Editá-la aqui altera a mesma entidade.
link-error-no-label = Uma ligação tem de dizer o que é. Nada foi guardado.
occupations-editor-title = Ocupações
occupations-editor-help = Uma ocupação é um período com um início e um fim, não um cargo. Cada uma traz as suas próprias datas e a sua própria fonte.
occupations-none = Não há nenhuma ocupação registada para esta pessoa.
occupations-new = Uma nova ocupação
occupations-create = Criar a ocupação
occupations-remove = Remover esta ocupação
occupations-title = O que fazia
occupations-employer = Para quem
occupations-employer-place = Onde estavam
occupations-from = Desde
occupations-until = Até
occupation-error-no-title = Uma ocupação tem de dizer o que alguém fazia. Nada foi guardado.
link-category-spiritual = espiritual
link-category-professional = profissional
link-category-social = social
link-category-legal = legal
link-category-medical = médica
link-category-educational = educativa
link-category-conflict = conflito
link-category-other = outra
links-edit-link = Editar ligações
occupations-edit-link = Editar ocupações
family-edit-link = Editar família e relações

## Events and documents

events-editor-title = Acontecimentos
events-editor-help = Um acontecimento nomeia várias pessoas ao mesmo tempo — um casamento, um baptizado, um recenseamento — por isso cada um é um registo próprio e aparece em cada página que nomeia.
events-none = Nenhum acontecimento nomeia esta pessoa.
events-new = Um novo acontecimento
events-new-help = Esta pessoa é acrescentada como sujeito se não nomear mais ninguém. Um acontecimento sem ninguém é apenas uma data.
events-create = Criar o acontecimento
events-remove = Remover este acontecimento
events-category = O que aconteceu
events-subcategory = Mais precisamente
events-description = Descrição
events-participants = Quem esteve
events-participants-help = Guardar altera o registo do acontecimento, que todas as outras pessoas nomeadas também mostram.
events-who = Quem
event-error-no-category = Um acontecimento tem de dizer o que aconteceu. Nada foi guardado.
documents-editor-title = Documentos
documents-editor-help = A que ficheiros este registo aponta e o que cada um é para ele. Limpar uma linha desliga o ficheiro: o documento e os seus bytes ficam no arquivo.
documents-attached = Anexados a este registo
documents-upload = Carregar um ficheiro
documents-upload-help = Até { $mb } MB. O ficheiro é guardado no arquivo e anexado a este registo.
documents-caption = Legenda
documents-edit-link = Anexar e desligar documentos
events-edit-link = Editar acontecimentos

## Presentation styles: density, never colour

prefs-style = Densidade
prefs-style-help = Quanto espaço a página ocupa. Separado do tema, que é só sobre cor, por isso qualquer combinação é possível.
style-comfortable = Confortável
style-comfortable-note = por omissão, com espaço para ler
style-compact = Compacta
style-compact-note = mais registo por ecrã, para percorrer vários
style-paper = Papel
style-paper-note = um tipo com serifas e filetes em vez de cartões, para ler com calma ou imprimir

## Sensitive classes

admin-export-choose = Incluir nesta exportação
scope-health = Saúde e crenças
scope-biometrics = Dados biométricos
scope-genomics = Dados genómicos
scope-legal = Registo criminal
scope-behaviour = Perfis comportamentais de pessoas vivas
admin-export-with-chosen = Exportar com o que está assinalado

## Profile

pg-identity = Identidade e estado civil
pg-identity-intro = Quem a pessoa era segundo os documentos e o que os registos civis assentaram.
pg-morphology = Morfologia
pg-morphology-intro = O corpo tal como foi medido e descrito.
pg-biometrics = Biometria
pg-biometrics-intro = A voz, as mãos e os sentidos, e os padrões que permitem identificar uma pessoa.
pg-health = Saúde
pg-health-intro = Doenças, tratamentos, medições e resultados.
pg-genomics = Genómica
pg-genomics-intro = Testes de ADN, haplogrupos, variantes e outros resultados moleculares.
pg-death = Óbito
pg-death-intro = Como, quando e onde terminou uma vida, e o que foi feito do corpo.
pg-residence = Residência e nacionalidade
pg-residence-intro = Onde a pessoa viveu, que Estados a tinham por nacional e que línguas falava.
pg-education = Educação e trabalho
pg-education-intro = Estudos, habilitações, rendimentos e bens.
pg-military = Serviço militar e distinções
pg-military-intro = Serviço, postos, unidades e condecorações.
pg-legal = Processos-crime
pg-legal-intro = Processos penais e o seu desfecho.
pg-belief = Crença e filiação
pg-belief-intro = Religião, ritos, convicções e filiações.
pg-personality = Personalidade e comportamento
pg-personality-intro = Temperamento, hábitos e passatempos, como as fontes os descrevem.
pg-relationships = Relações
pg-relationships-intro = Pais, cônjuges, filhos e as outras pessoas de uma vida.
pg-digital-legacy = Legado digital
pg-digital-legacy-intro = Digitalizações, modelos, gravações e arquivos que representam uma pessoa.
pa-identity-titles = Títulos
pa-identity-sex-at-birth = Sexo à nascença
pa-identity-gender-identity = Identidade de género
pa-birth-time = Hora do nascimento
pa-birth-coordinates = Local de nascimento em coordenadas
pa-civil-status-birth-certificate-number = Número do assento de nascimento
pa-civil-status-register-entries = Assentos de registo civil
pa-civil-status-marginal-annotations = Averbamentos
pa-morphology-height = Altura
pa-morphology-weight = Peso
pa-morphology-bmi = Índice de massa corporal
pa-morphology-body-composition = Composição corporal
pa-morphology-build = Constituição física
pa-morphology-eye-colour = Cor dos olhos
pa-morphology-eye-shape = Forma dos olhos
pa-morphology-eye-spacing = Afastamento dos olhos
pa-morphology-hair-colour = Cor natural do cabelo
pa-morphology-hair-texture = Tipo de cabelo
pa-morphology-hairline = Linha do cabelo
pa-morphology-facial-hair = Pelos faciais
pa-morphology-body-hair = Pelos do corpo
pa-morphology-skin-tone = Fototipo (Fitzpatrick)
pa-morphology-skin-undertone = Subtom da pele
pa-morphology-freckles = Sardas
pa-morphology-pigmentation = Manchas de pigmentação
pa-morphology-scars = Cicatrizes
pa-morphology-tattoos = Tatuagens
pa-morphology-moles = Sinais
pa-morphology-facial-asymmetries = Assimetrias faciais
pa-morphology-face-shape = Forma do rosto
pa-morphology-nose-shape = Forma do nariz
pa-morphology-ear-shape = Forma das orelhas
pa-morphology-lip-shape = Forma dos lábios
pa-morphology-dentition = Dentição
pa-morphology-malocclusion = Má oclusão (classe de Angle)
pa-morphology-posture = Postura
pa-morphology-gait = Marcha
pa-morphology-distinguishing-features = Sinais particulares
pa-biometrics-fingerprints = Impressões digitais
pa-biometrics-retinal-print = Padrão da retina
pa-biometrics-voice-signature = Impressão vocal
pa-biometrics-voice-frequency = Frequência fundamental da voz
pa-biometrics-vocal-timbre = Timbre de voz
pa-biometrics-spoken-accent = Sotaque
pa-biometrics-speech-rate = Ritmo de fala
pa-biometrics-verbal-tics = Bordões
pa-biometrics-frequent-vocabulary = Vocabulário frequente
pa-biometrics-speech-register = Registo de fala
pa-biometrics-motor-tics = Tiques motores
pa-biometrics-handedness = Lateralidade manual
pa-biometrics-hearing = Audição
pa-biometrics-visual-acuity = Acuidade visual
pa-biometrics-optical-correction = Correção ótica
pa-health-blood-group = Grupo sanguíneo (AB0)
pa-health-rhesus = Fator Rh (RhD)
pa-health-blood-pressure = Tensão arterial
pa-health-resting-heart-rate = Frequência cardíaca em repouso
pa-health-respiratory-capacity = Função respiratória
pa-health-conditions = Doenças
pa-health-surgeries = Antecedentes cirúrgicos
pa-health-injuries = Lesões
pa-health-deformities = Deformidades
pa-health-amputations = Amputações
pa-health-prostheses = Próteses
pa-health-implants = Implantes
pa-health-devices = Dispositivos implantados
pa-health-medications = Medicamentos
pa-health-allergies = Alergias
pa-health-vaccinations = Vacinas
pa-health-serology = Serologia
pa-health-lab-results = Análises clínicas
pa-health-deficiencies = Carências
pa-health-sleep-disorders = Perturbações do sono
pa-health-mental-health-assessments = Avaliações de saúde mental
pa-genomics-autosomal-mapping = Teste de ADN autossómico
pa-genomics-y-haplogroup = Haplogrupo do cromossoma Y
pa-genomics-mt-haplogroup = Haplogrupo mitocondrial
pa-genomics-whole-genome-sequencing = Sequenciação do genoma completo
pa-genomics-risk-variants = Variantes de risco
pa-genomics-hereditary-conditions = Doenças hereditárias
pa-genomics-predispositions = Predisposições
pa-genomics-epigenetic-markers = Marcadores epigenéticos
pa-genomics-epigenetic-age = Idade epigenética
pa-genomics-gut-microbiome = Microbioma intestinal
pa-genomics-skin-microbiome = Microbioma cutâneo
pa-genomics-toxicological-sensitivities = Sensibilidade a fármacos e tóxicos
pa-death-time = Hora do óbito
pa-death-coordinates = Local do óbito em coordenadas
pa-death-causes = Causas de morte
pa-death-contributing-factors = Fatores contributivos
pa-death-autopsy = Autópsia
pa-death-disposition = Destino do corpo
pa-death-grave = Sepultura
pa-residence-addresses = Moradas
pa-residence-nationality-of-origin = Nacionalidade de origem
pa-residence-acquired-nationalities = Nacionalidades adquiridas
pa-residence-mother-tongue = Língua materna
pa-residence-spoken-languages = Línguas faladas
pa-education-level = Nível de escolaridade
pa-education-diplomas = Diplomas e graus
pa-education-institutions = Escolas e instituições
pa-education-income = Rendimento
pa-education-real-estate = Imóveis
pa-military-distinctions = Condecorações
pa-military-citations = Louvores
pa-military-ranks = Postos
pa-military-units = Unidades
pa-military-service-numbers = Números de matrícula
pa-legal-criminal-record = Registo criminal
pa-belief-religions = Religião
pa-belief-sacraments = Sacramentos e ritos
pa-belief-beliefs = Convicções
pa-belief-political-leanings = Tendência política
pa-belief-memberships = Filiações
pa-personality-big-five = Pontuações Big Five
pa-personality-mbti = Tipo MBTI
pa-personality-introversion-extraversion = Introversão e extroversão
pa-personality-stress-tolerance = Tolerância ao stress
pa-personality-decision-style = Estilo de decisão
pa-personality-interests = Interesses
pa-personality-hobbies = Passatempos
pa-personality-sports = Desporto
pa-personality-dietary-habits = Alimentação
pa-personality-dependencies = Dependências
pa-digital-legacy-body-models = Modelos do corpo
pa-digital-legacy-skin-textures = Texturas da pele
pa-digital-legacy-rigs = Esqueletos de animação
pa-digital-legacy-voice-corpora = Gravações para síntese de voz
pa-digital-legacy-text-corpora = Escritos para um modelo de linguagem
pa-digital-legacy-digital-traces = Rasto digital
pa-digital-legacy-carbon-footprint = Pegada de carbono
pa-digital-legacy-behaviour-models = Modelos de comportamento
pf-identity-titles-text = Título como consta
pf-identity-titles-kind = Tipo de título
pf-civil-status-marginal-annotations-text = Averbamento
pf-morphology-pigmentation-kind = Tipo de mancha
pf-biometrics-spoken-accent-description = Como é descrito
pf-biometrics-optical-correction-kind = Correção
pf-health-amputations-level = Nível da amputação
pf-health-prostheses-kind = Prótese
pf-health-implants-kind = Implante
pf-health-devices-kind = Dispositivo
pf-health-allergies-type = Tipo de alergia
pf-health-vaccinations-status = Estado vacinal
pf-health-sleep-disorders-category = Categoria da perturbação
pf-death-autopsy-kind = Autópsia
pf-education-institutions-name = Nome da instituição
pf-military-distinctions-name = Nome da condecoração
pf-military-distinctions-kind = Tipo de condecoração
pf-military-citations-text = Texto do louvor
pf-military-ranks-category = Categoria do posto
pf-belief-political-leanings-position = Posição no eixo esquerda–direita
pf-belief-memberships-kind = Tipo de organização
pf-digital-legacy-carbon-footprint-method = Método de estimativa
pf-age-years = Idade em anos
pf-agreeableness = Amabilidade
pf-allergen = Alergénio
pf-amount = Montante
pf-analyte = Analito
pf-artefact-type = Tipo de artefacto
pf-autoimmune = Autoimune
pf-body-region = Zona do corpo
pf-bone-percent = Osso
pf-carrier-status = Estado de portador
pf-cause = Causa
pf-chronic = Crónica
pf-clock = Relógio
pf-condition = Doença
pf-conferred-by = Atribuída por
pf-congenital = Congénita
pf-conscientiousness = Conscienciosidade
pf-consent = Consentimento
pf-coordinates = Coordenadas
pf-corrected = Com correção
pf-country = País
pf-court = Tribunal
pf-coverage = Cobertura
pf-currency = Moeda
pf-decimal = Acuidade (decimal)
pf-denomination = Confissão
pf-derived-from-id = Derivado de
pf-description = Descrição
pf-details = Pormenores
pf-diagnosis = Diagnóstico
pf-diameter-mm = Diâmetro
pf-diastolic = Diastólica
pf-diet = Dieta
pf-document-id = Documento
pf-dose = Dose
pf-ear = Ouvido
pf-entry-number = Número do assento
pf-extraversion = Extroversão
pf-eye = Olho
pf-fat-percent = Gordura
pf-fev1-fvc-ratio = Razão FEV1/FVC
pf-fev1-litres = FEV1
pf-file-format = Formato do ficheiro
pf-findings = Conclusões
pf-flag = Sinalização
pf-format = Formato
pf-fracture = Fratura
pf-fvc-litres = FVC
pf-gene = Gene
pf-generator = Feito com
pf-grade = Grau
pf-iccs-section = Secção do crime (ICCS)
pf-icd10-chapter = Capítulo da CID-10
pf-indication = Indicação
pf-inheritance = Transmissão
pf-inscription = Inscrição
pf-institution = Instituição
pf-instrument = Instrumento
pf-isced-level = Nível CITE
pf-jurisdiction = Jurisdição
pf-language = Língua
pf-lat = Latitude
pf-level = Nível
pf-lines = Morada
pf-location = Localização
pf-lon = Longitude
pf-major = Haplogrupo principal
pf-marker = Marcador
pf-metaboliser-status = Fenótipo metabolizador
pf-method = Método
pf-mode = Forma de aquisição
pf-muscle-percent = Músculo
pf-neuroticism = Neuroticismo
pf-number = Número
pf-nutrient = Nutriente
pf-offence = Crime
pf-office = Conservatória
pf-openness = Abertura
pf-organisation = Organização
pf-outcome = Desfecho
pf-pace = Ritmo de envelhecimento
pf-page = Folha
pf-panel = Painel
pf-party = Partido
pf-pathogen = Agente patogénico
pf-pattern = Padrão de consumo
pf-percentile = Percentil
pf-period = Periodicidade
pf-place-id = Local
pf-plot = Talhão
pf-polygenic-score = Pontuação poligénica
pf-postal-code = Código postal
pf-precision = Precisão
pf-prescription = Graduação
pf-proficiency = Nível de domínio
pf-provider = Fornecedor
pf-quintile = Quintil de rendimento
pf-rank = Posto
pf-rank-text = Posto como consta
pf-reaction = Reação
pf-reference-build = Genoma de referência
pf-reference-high = Limite superior de referência
pf-reference-low = Limite inferior de referência
pf-register-type = Tipo de assento
pf-result = Resultado
pf-role = Cargo
pf-sacrament = Sacramento ou rito
pf-score = Pontuação
pf-sentence = Pena
pf-sequence = Lugar na cadeia de causas
pf-service = Ramo
pf-severity = Gravidade
pf-shannon-diversity = Diversidade de Shannon
pf-shape = Forma
pf-significance = Significado clínico
pf-snp-count = SNP analisados
pf-sport = Modalidade
pf-subclade = Subclado
pf-substance = Substância
pf-summary = Resumo
pf-systolic = Sistólica
pf-tenure = Regime de posse
pf-test = Teste
pf-threshold-db = Limiar auditivo
pf-title = Designação
pf-tonnes-co2e-per-year = Emissões
pf-tradition = Tradição
pf-tree-version = Versão da árvore
pf-unit = Unidade
pf-use = Utilização
pf-variant = Variante
pf-volume = Livro
pf-zygosity = Zigosidade
pu-cm = { $n } cm
pu-kg = { $n } kg
pu-kg-m2 = { $n } kg/m²
pu-percent = { $n } %
pu-mm = { $n } mm
pu-hz = { $n } Hz
pu-words-min = { $n } palavras/min
pu-db-hl = { $n } dB HL
pu-mmhg = { $n } mmHg
pu-bpm = { $n } bpm
pu-litres = { $n } L
pu-coverage = { $n }×
pu-years = { $n } anos
pu-t-co2e-yr = { $n } t CO₂e por ano
pv-sensitive-class-health = Saúde e crenças
pv-sensitive-class-biometrics = Dados biométricos
pv-sensitive-class-genomics = Dados genómicos
pv-sensitive-class-legal = Registo criminal
pv-laterality-left = Esquerdo
pv-laterality-right = Direito
pv-laterality-both = Ambos
pv-body-region-head = Cabeça
pv-body-region-face = Rosto
pv-body-region-neck = Pescoço
pv-body-region-left-shoulder = Ombro esquerdo
pv-body-region-right-shoulder = Ombro direito
pv-body-region-left-arm = Braço esquerdo
pv-body-region-right-arm = Braço direito
pv-body-region-left-hand = Mão esquerda
pv-body-region-right-hand = Mão direita
pv-body-region-chest = Tórax
pv-body-region-abdomen = Abdómen
pv-body-region-upper-back = Parte superior das costas
pv-body-region-lower-back = Parte inferior das costas
pv-body-region-pelvis = Bacia e ancas
pv-body-region-left-leg = Perna esquerda
pv-body-region-right-leg = Perna direita
pv-body-region-left-foot = Pé esquerdo
pv-body-region-right-foot = Pé direito
pv-body-region-internal = Interna
pv-body-region-whole-body = Todo o corpo
pv-body-region-other = Outra zona
pv-artefact-type-mesh = Malha
pv-artefact-type-point-cloud = Nuvem de pontos
pv-artefact-type-skin-texture-map = Mapa de textura da pele
pv-artefact-type-skeletal-rig = Esqueleto de animação
pv-artefact-type-voice-corpus = Corpus de voz
pv-artefact-type-text-corpus = Corpus de textos
pv-artefact-type-trace-archive = Arquivo de atividade online
pv-artefact-type-behaviour-model = Modelo de comportamento
pv-artefact-type-fingerprint-card = Ficha datiloscópica
pv-artefact-type-fingerprint-template = Modelo de impressão digital
pv-artefact-type-retinal-image = Imagem da retina
pv-artefact-type-voiceprint = Impressão vocal
pv-consent-given = Dado
pv-consent-given-by-estate = Dado pelos herdeiros
pv-consent-refused = Recusado
pv-consent-withdrawn = Retirado
pv-consent-not-asked = Não pedido
pv-consent-unknown = Desconhecido
pv-sex-at-birth-female = Feminino
pv-sex-at-birth-male = Masculino
pv-sex-at-birth-intersex = Intersexo
pv-sex-at-birth-undetermined = Indeterminado
pv-sex-at-birth-unknown = Desconhecido
pv-gender-identity-woman = Mulher
pv-gender-identity-man = Homem
pv-gender-identity-non-binary = Não binária
pv-gender-identity-other = Outra
pv-gender-identity-undisclosed = Não revelada
pv-gender-identity-unknown = Desconhecida
pv-title-kind-nobility = Nobiliárquico
pv-title-kind-academic = Académico
pv-title-kind-professional = Profissional
pv-title-kind-religious = Religioso
pv-title-kind-military = Militar
pv-title-kind-civic = Honorífico
pv-title-kind-courtesy = De cortesia
pv-title-kind-other = Outro
pv-register-type-birth = Nascimento
pv-register-type-baptism = Batismo
pv-register-type-marriage = Casamento
pv-register-type-death = Óbito
pv-register-type-burial = Enterro
pv-register-type-divorce = Divórcio
pv-register-type-recognition = Perfilhação
pv-register-type-legitimation = Legitimação
pv-register-type-adoption = Adoção
pv-register-type-name-change = Alteração de nome
pv-register-type-other = Outro
pv-build-slight = Franzina
pv-build-slim = Magra
pv-build-average = Média
pv-build-sturdy = Robusta
pv-build-stout = Encorpada
pv-build-heavy = Pesada
pv-eye-colour-light-blue = Azul-claro
pv-eye-colour-blue = Azul
pv-eye-colour-dark-blue = Azul-escuro
pv-eye-colour-grey = Cinzento
pv-eye-colour-blue-grey = Azul-acinzentado
pv-eye-colour-green = Verde
pv-eye-colour-grey-green = Verde-acinzentado
pv-eye-colour-hazel = Avelã
pv-eye-colour-amber = Âmbar
pv-eye-colour-light-brown = Castanho-claro
pv-eye-colour-brown = Castanho
pv-eye-colour-dark-brown = Castanho-escuro
pv-eye-colour-black = Preto
pv-eye-colour-mixed = Misto
pv-eye-colour-other = Outro
pv-eye-shape-almond = Amendoados
pv-eye-shape-round = Redondos
pv-eye-shape-hooded = Pálpebra descaída
pv-eye-shape-monolid = Sem prega palpebral
pv-eye-shape-deep-set = Encovados
pv-eye-shape-protruding = Salientes
pv-eye-shape-upturned = Para cima
pv-eye-shape-downturned = Para baixo
pv-eye-shape-other = Outra
pv-eye-spacing-close-set = Juntos
pv-eye-spacing-average = Normal
pv-eye-spacing-wide-set = Afastados
pv-hair-colour-black = Preto
pv-hair-colour-dark-brown = Castanho-escuro
pv-hair-colour-brown = Castanho
pv-hair-colour-light-brown = Castanho-claro
pv-hair-colour-auburn = Acobreado
pv-hair-colour-red = Ruivo
pv-hair-colour-strawberry-blond = Louro-arruivado
pv-hair-colour-dark-blond = Louro-escuro
pv-hair-colour-blond = Louro
pv-hair-colour-light-blond = Louro-claro
pv-hair-colour-grey = Grisalho
pv-hair-colour-white = Branco
pv-hair-colour-none = Sem cabelo
pv-hair-colour-other = Outro
pv-hair-texture-straight = Liso
pv-hair-texture-wavy = Ondulado
pv-hair-texture-curly = Encaracolado
pv-hair-texture-coily = Crespo
pv-hair-texture-other = Outro
pv-hairline-straight = Reta
pv-hairline-rounded = Arredondada
pv-hairline-widows-peak = Em bico
pv-hairline-m-shaped = Em M
pv-hairline-bell-shaped = Em sino
pv-hairline-uneven = Irregular
pv-hairline-receding = Com entradas
pv-hairline-bald = Calvície
pv-facial-hair-none = Nenhum
pv-facial-hair-stubble = Barba por fazer
pv-facial-hair-moustache = Bigode
pv-facial-hair-goatee = Pera
pv-facial-hair-full-beard = Barba cheia
pv-facial-hair-sideburns = Patilhas
pv-facial-hair-other = Outro
pv-body-hair-none = Nenhum
pv-body-hair-sparse = Escasso
pv-body-hair-moderate = Moderado
pv-body-hair-dense = Abundante
pv-skin-tone-type-i = Tipo I — queima sempre, nunca bronzeia
pv-skin-tone-type-ii = Tipo II — queima com facilidade, bronzeia pouco
pv-skin-tone-type-iii = Tipo III — queima às vezes, bronzeia uniformemente
pv-skin-tone-type-iv = Tipo IV — raramente queima, bronzeia bem
pv-skin-tone-type-v = Tipo V — muito raramente queima
pv-skin-tone-type-vi = Tipo VI — nunca queima
pv-skin-undertone-cool = Frio
pv-skin-undertone-neutral = Neutro
pv-skin-undertone-warm = Quente
pv-skin-undertone-olive = Oliváceo
pv-freckles-none = Nenhumas
pv-freckles-few = Poucas
pv-freckles-moderate = Moderadas
pv-freckles-many = Muitas
pv-pigmentation-mark-birthmark = Sinal de nascença
pv-pigmentation-mark-port-wine-stain = Mancha vinho do Porto
pv-pigmentation-mark-cafe-au-lait-spot = Mancha café com leite
pv-pigmentation-mark-depigmented-patch = Mancha despigmentada
pv-pigmentation-mark-hyperpigmented-patch = Mancha hiperpigmentada
pv-pigmentation-mark-other = Outra
pv-mole-shape-round = Redondo
pv-mole-shape-oval = Oval
pv-mole-shape-irregular = Irregular
pv-mole-shape-other = Outra
pv-face-shape-oval = Oval
pv-face-shape-round = Redondo
pv-face-shape-square = Quadrado
pv-face-shape-oblong = Alongado
pv-face-shape-heart = Em coração
pv-face-shape-diamond = Em losango
pv-face-shape-triangular = Triangular
pv-nose-shape-straight = Reto
pv-nose-shape-aquiline = Aquilino
pv-nose-shape-snub = Achatado e curto
pv-nose-shape-upturned = Arrebitado
pv-nose-shape-flat = Achatado
pv-nose-shape-broad = Largo
pv-nose-shape-bulbous = Bulboso
pv-nose-shape-crooked = Torto
pv-nose-shape-other = Outra
pv-ear-shape-free-lobe = Lóbulos soltos
pv-ear-shape-attached-lobe = Lóbulos presos
pv-ear-shape-protruding = Salientes
pv-ear-shape-close-set = Coladas
pv-ear-shape-pointed = Pontiagudas
pv-ear-shape-other = Outra
pv-lip-shape-thin = Finos
pv-lip-shape-medium = Médios
pv-lip-shape-full = Carnudos
pv-lip-shape-bow-shaped = Em arco
pv-lip-shape-wide = Largos
pv-lip-shape-downturned = Descaídos
pv-lip-shape-other = Outra
pv-dentition-primary = Decídua
pv-dentition-mixed = Mista
pv-dentition-permanent-complete = Permanente, completa
pv-dentition-permanent-partial-loss = Permanente, incompleta
pv-dentition-edentulous = Desdentado
pv-dentition-partial-denture = Prótese parcial
pv-dentition-full-denture = Prótese total
pv-dentition-implants = Implantes dentários
pv-malocclusion-normal = Oclusão normal
pv-malocclusion-class-i = Classe I
pv-malocclusion-class-ii-division-1 = Classe II, divisão 1
pv-malocclusion-class-ii-division-2 = Classe II, divisão 2
pv-malocclusion-class-iii = Classe III
pv-posture-ideal = Correta
pv-posture-kyphotic-lordotic = Cifolordótica
pv-posture-flat-back = Costas planas
pv-posture-sway-back = Costas arqueadas
pv-posture-stooped = Curvada
pv-posture-scoliotic = Escoliótica
pv-posture-other = Outra
pv-gait-brisk = Rápida
pv-gait-average = Normal
pv-gait-slow = Lenta
pv-gait-shuffling = Arrastada
pv-gait-limping = Coxeante
pv-gait-waddling = Bamboleante
pv-gait-unsteady = Instável
pv-gait-stiff = Rígida
pv-gait-other = Outra
pv-vocal-timbre-bright = Claro
pv-vocal-timbre-dark = Escuro
pv-vocal-timbre-warm = Caloroso
pv-vocal-timbre-breathy = Soprado
pv-vocal-timbre-nasal = Nasal
pv-vocal-timbre-hoarse = Rouco
pv-vocal-timbre-resonant = Ressonante
pv-vocal-timbre-thin = Fino
pv-vocal-timbre-other = Outro
pv-speech-register-frozen = Solene
pv-speech-register-formal = Formal
pv-speech-register-consultative = Neutro
pv-speech-register-casual = Coloquial
pv-speech-register-intimate = Íntimo
pv-handedness-left = Canhoto
pv-handedness-right = Destro
pv-handedness-ambidextrous = Ambidestro
pv-handedness-mixed = Mista
pv-handedness-unknown = Desconhecida
pv-hearing-grade-normal = Normal
pv-hearing-grade-mild = Ligeira
pv-hearing-grade-moderate = Moderada
pv-hearing-grade-moderately-severe = Moderadamente grave
pv-hearing-grade-severe = Grave
pv-hearing-grade-profound = Profunda
pv-hearing-grade-complete = Total
pv-optical-correction-none = Nenhuma
pv-optical-correction-glasses = Óculos
pv-optical-correction-contact-lenses = Lentes de contacto
pv-optical-correction-glasses-and-contact-lenses = Óculos e lentes de contacto
pv-optical-correction-refractive-surgery = Cirurgia refrativa
pv-optical-correction-intraocular-lens = Lente intraocular
pv-optical-correction-other = Outra
pv-rhesus-positive = RhD positivo
pv-rhesus-negative = RhD negativo
pv-rhesus-weak-d = D fraco
pv-rhesus-unknown = Desconhecido
pv-icd10-chapter-infectious-parasitic = I Doenças infecciosas e parasitárias
pv-icd10-chapter-neoplasms = II Neoplasias
pv-icd10-chapter-blood-immune = III Sangue e sistema imunitário
pv-icd10-chapter-endocrine-metabolic = IV Endócrinas, nutricionais e metabólicas
pv-icd10-chapter-mental-behavioural = V Perturbações mentais e comportamentais
pv-icd10-chapter-nervous-system = VI Sistema nervoso
pv-icd10-chapter-eye-adnexa = VII Olho e anexos
pv-icd10-chapter-ear-mastoid = VIII Ouvido e apófise mastoide
pv-icd10-chapter-circulatory = IX Aparelho circulatório
pv-icd10-chapter-respiratory = X Aparelho respiratório
pv-icd10-chapter-digestive = XI Aparelho digestivo
pv-icd10-chapter-skin = XII Pele e tecido subcutâneo
pv-icd10-chapter-musculoskeletal = XIII Sistema osteomuscular
pv-icd10-chapter-genitourinary = XIV Aparelho geniturinário
pv-icd10-chapter-pregnancy-childbirth = XV Gravidez e parto
pv-icd10-chapter-perinatal = XVI Afeções perinatais
pv-icd10-chapter-congenital = XVII Malformações congénitas
pv-icd10-chapter-ill-defined = XVIII Sintomas e causas mal definidas
pv-icd10-chapter-injury-poisoning = XIX Lesões e envenenamentos
pv-icd10-chapter-external-causes = XX Causas externas
pv-icd10-chapter-health-factors = XXI Fatores que influenciam a saúde
pv-icd10-chapter-special-purposes = XXII Códigos para fins especiais
pv-diagnosis-status-diagnosed = Diagnosticada
pv-diagnosis-status-suspected = Suspeita
pv-diagnosis-status-self-reported = Referida
pv-diagnosis-status-unknown = Desconhecido
pv-prosthesis-kind-limb = Membro
pv-prosthesis-kind-joint = Articular
pv-prosthesis-kind-ocular = Ocular
pv-prosthesis-kind-dental = Dentária
pv-prosthesis-kind-auditory = Auditiva
pv-prosthesis-kind-breast = Mamária
pv-prosthesis-kind-other = Outra
pv-implant-kind-orthopaedic = Ortopédico
pv-implant-kind-dental = Dentário
pv-implant-kind-cochlear = Coclear
pv-implant-kind-breast = Mamário
pv-implant-kind-intraocular-lens = Lente intraocular
pv-implant-kind-contraceptive = Contracetivo
pv-implant-kind-cosmetic = Estético
pv-implant-kind-other = Outro
pv-device-kind-pacemaker = Pacemaker
pv-device-kind-implantable-defibrillator = Desfibrilhador implantável
pv-device-kind-cardiac-resynchronisation = Dispositivo de ressincronização
pv-device-kind-ventricular-assist = Assistência ventricular
pv-device-kind-neurostimulator = Neuroestimulador
pv-device-kind-insulin-pump = Bomba de insulina
pv-device-kind-drug-port = Cateter totalmente implantado
pv-device-kind-shunt = Derivação
pv-device-kind-stent = Stent
pv-device-kind-other = Outro
pv-allergy-type-drug = Medicamento
pv-allergy-type-food = Alimento
pv-allergy-type-environmental = Ambiental
pv-allergy-type-insect-venom = Veneno de inseto
pv-allergy-type-latex = Látex
pv-allergy-type-other = Outro
pv-allergy-severity-mild = Ligeira
pv-allergy-severity-moderate = Moderada
pv-allergy-severity-severe = Grave
pv-allergy-severity-anaphylactic = Anafilática
pv-allergy-severity-unknown = Desconhecida
pv-pathogen-diphtheria = Difteria
pv-pathogen-tetanus = Tétano
pv-pathogen-pertussis = Tosse convulsa
pv-pathogen-poliomyelitis = Poliomielite
pv-pathogen-measles = Sarampo
pv-pathogen-mumps = Papeira
pv-pathogen-rubella = Rubéola
pv-pathogen-varicella = Varicela
pv-pathogen-smallpox = Varíola
pv-pathogen-tuberculosis = Tuberculose
pv-pathogen-hepatitis-a = Hepatite A
pv-pathogen-hepatitis-b = Hepatite B
pv-pathogen-hepatitis-c = Hepatite C
pv-pathogen-haemophilus-influenzae-b = Haemophilus influenzae tipo b
pv-pathogen-pneumococcal = Pneumococo
pv-pathogen-meningococcal = Meningococo
pv-pathogen-human-papillomavirus = Vírus do papiloma humano
pv-pathogen-influenza = Gripe
pv-pathogen-covid-19 = COVID-19
pv-pathogen-rotavirus = Rotavírus
pv-pathogen-yellow-fever = Febre-amarela
pv-pathogen-typhoid = Febre tifoide
pv-pathogen-cholera = Cólera
pv-pathogen-rabies = Raiva
pv-pathogen-japanese-encephalitis = Encefalite japonesa
pv-pathogen-tick-borne-encephalitis = Encefalite transmitida por carraças
pv-pathogen-hiv = VIH
pv-pathogen-syphilis = Sífilis
pv-pathogen-toxoplasmosis = Toxoplasmose
pv-pathogen-cytomegalovirus = Citomegalovírus
pv-pathogen-epstein-barr = Vírus Epstein-Barr
pv-pathogen-other = Outro
pv-vaccination-status-vaccinated = Vacinado
pv-vaccination-status-partially-vaccinated = Parcialmente vacinado
pv-vaccination-status-unvaccinated = Não vacinado
pv-vaccination-status-contraindicated = Contraindicada
pv-vaccination-status-unknown = Desconhecido
pv-serology-result-positive = Positivo
pv-serology-result-negative = Negativo
pv-serology-result-equivocal = Duvidoso
pv-serology-result-unknown = Desconhecido
pv-lab-panel-basic-metabolic = Perfil metabólico básico
pv-lab-panel-lipid = Perfil lipídico
pv-lab-panel-liver = Perfil hepático
pv-lab-panel-renal = Perfil renal
pv-lab-panel-glycated-haemoglobin = Hemoglobina glicada
pv-lab-panel-iron = Metabolismo do ferro
pv-lab-analyte-sodium = Sódio
pv-lab-analyte-potassium = Potássio
pv-lab-analyte-chloride = Cloro
pv-lab-analyte-bicarbonate = Bicarbonato
pv-lab-analyte-urea = Ureia
pv-lab-analyte-creatinine = Creatinina
pv-lab-analyte-glucose = Glicose
pv-lab-analyte-calcium = Cálcio
pv-lab-analyte-total-cholesterol = Colesterol total
pv-lab-analyte-ldl-cholesterol = Colesterol LDL
pv-lab-analyte-hdl-cholesterol = Colesterol HDL
pv-lab-analyte-triglycerides = Triglicéridos
pv-lab-analyte-non-hdl-cholesterol = Colesterol não HDL
pv-lab-analyte-alt = Alanina aminotransferase (ALT)
pv-lab-analyte-ast = Aspartato aminotransferase (AST)
pv-lab-analyte-alp = Fosfatase alcalina (FA)
pv-lab-analyte-ggt = Gama-glutamiltransferase (GGT)
pv-lab-analyte-total-bilirubin = Bilirrubina total
pv-lab-analyte-direct-bilirubin = Bilirrubina direta
pv-lab-analyte-albumin = Albumina
pv-lab-analyte-total-protein = Proteínas totais
pv-lab-analyte-egfr = TFG estimada
pv-lab-analyte-uric-acid = Ácido úrico
pv-lab-analyte-phosphate = Fosfato
pv-lab-analyte-urine-albumin-creatinine-ratio = Razão albumina/creatinina urinária
pv-lab-analyte-hba1c = HbA1c
pv-lab-analyte-serum-iron = Ferro sérico
pv-lab-analyte-ferritin = Ferritina
pv-lab-analyte-transferrin = Transferrina
pv-lab-analyte-transferrin-saturation = Saturação da transferrina
pv-lab-analyte-tibc = Capacidade total de fixação do ferro
pv-lab-flag-low = Baixo
pv-lab-flag-normal = Normal
pv-lab-flag-high = Alto
pv-lab-flag-critical-low = Criticamente baixo
pv-lab-flag-critical-high = Criticamente alto
pv-nutrient-vitamin-a = Vitamina A
pv-nutrient-thiamine = Tiamina (B1)
pv-nutrient-riboflavin = Riboflavina (B2)
pv-nutrient-niacin = Niacina (B3)
pv-nutrient-vitamin-b6 = Vitamina B6
pv-nutrient-folate = Folato (B9)
pv-nutrient-vitamin-b12 = Vitamina B12
pv-nutrient-vitamin-c = Vitamina C
pv-nutrient-vitamin-d = Vitamina D
pv-nutrient-vitamin-e = Vitamina E
pv-nutrient-vitamin-k = Vitamina K
pv-nutrient-iron = Ferro
pv-nutrient-zinc = Zinco
pv-nutrient-magnesium = Magnésio
pv-nutrient-calcium = Cálcio
pv-nutrient-iodine = Iodo
pv-nutrient-selenium = Selénio
pv-nutrient-copper = Cobre
pv-nutrient-potassium = Potássio
pv-nutrient-phosphorus = Fósforo
pv-nutrient-other = Outro
pv-sleep-disorder-insomnia = Insónia
pv-sleep-disorder-sleep-related-breathing = Perturbação respiratória do sono
pv-sleep-disorder-central-hypersomnolence = Hipersónia central
pv-sleep-disorder-circadian-rhythm = Perturbação do ritmo circadiano
pv-sleep-disorder-parasomnia = Parassónia
pv-sleep-disorder-sleep-related-movement = Perturbação do movimento no sono
pv-sleep-disorder-other = Outra
pv-assessment-instrument-phq-9 = PHQ-9
pv-assessment-instrument-gad-7 = GAD-7
pv-assessment-instrument-bdi-ii = BDI-II
pv-assessment-instrument-hads = HADS
pv-assessment-instrument-k10 = K10
pv-assessment-instrument-gds-15 = GDS-15
pv-assessment-instrument-mmse = MMSE
pv-assessment-instrument-moca = MoCA
pv-assessment-instrument-audit = AUDIT
pv-assessment-instrument-clinical-interview = Entrevista clínica
pv-assessment-instrument-other = Outro
pv-assessment-severity-none-minimal = Nula ou mínima
pv-assessment-severity-mild = Ligeira
pv-assessment-severity-moderate = Moderada
pv-assessment-severity-moderately-severe = Moderadamente grave
pv-assessment-severity-severe = Grave
pv-reference-build-grch36 = GRCh36 (hg18)
pv-reference-build-grch37 = GRCh37 (hg19)
pv-reference-build-grch38 = GRCh38 (hg38)
pv-reference-build-t2t-chm13 = T2T-CHM13
pv-genomic-file-format-raw-microarray = Dados brutos de microarray
pv-genomic-file-format-fastq = FASTQ
pv-genomic-file-format-bam = BAM
pv-genomic-file-format-cram = CRAM
pv-genomic-file-format-vcf = VCF
pv-genomic-file-format-gvcf = gVCF
pv-genomic-file-format-other = Outro
pv-zygosity-heterozygous = Heterozigótico
pv-zygosity-homozygous = Homozigótico
pv-zygosity-hemizygous = Hemizigótico
pv-zygosity-compound-heterozygous = Heterozigótico composto
pv-clinical-significance-pathogenic = Patogénica
pv-clinical-significance-likely-pathogenic = Provavelmente patogénica
pv-clinical-significance-uncertain-significance = Significado incerto
pv-clinical-significance-likely-benign = Provavelmente benigna
pv-clinical-significance-benign = Benigna
pv-inheritance-pattern-autosomal-dominant = Autossómica dominante
pv-inheritance-pattern-autosomal-recessive = Autossómica recessiva
pv-inheritance-pattern-x-linked-dominant = Dominante ligada ao X
pv-inheritance-pattern-x-linked-recessive = Recessiva ligada ao X
pv-inheritance-pattern-y-linked = Ligada ao Y
pv-inheritance-pattern-mitochondrial = Mitocondrial
pv-inheritance-pattern-multifactorial = Multifatorial
pv-inheritance-pattern-unknown = Desconhecida
pv-carrier-status-affected = Afetado
pv-carrier-status-carrier = Portador
pv-carrier-status-not-carrier = Não portador
pv-carrier-status-unknown = Desconhecido
pv-epigenetic-clock-horvath = Horvath
pv-epigenetic-clock-hannum = Hannum
pv-epigenetic-clock-phenoage = PhenoAge
pv-epigenetic-clock-grimage = GrimAge
pv-epigenetic-clock-dunedinpace = DunedinPACE
pv-epigenetic-clock-other = Outro
pv-metaboliser-status-poor = Metabolizador lento
pv-metaboliser-status-intermediate = Metabolizador intermédio
pv-metaboliser-status-normal = Metabolizador normal
pv-metaboliser-status-rapid = Metabolizador rápido
pv-metaboliser-status-ultrarapid = Metabolizador ultrarrápido
pv-autopsy-not-performed = Não realizada
pv-autopsy-clinical = Clínica
pv-autopsy-forensic = Médico-legal
pv-autopsy-external-examination = Apenas exame externo
pv-autopsy-unknown = Desconhecido
pv-disposition-burial = Inumação
pv-disposition-cremation = Cremação
pv-disposition-entombment = Sepultura em jazigo
pv-disposition-burial-at-sea = Sepultura no mar
pv-disposition-natural-burial = Enterro natural
pv-disposition-body-donation = Doação do corpo à ciência
pv-disposition-other = Outro
pv-disposition-unknown = Desconhecido
pv-address-use-principal = Residência habitual
pv-address-use-secondary = Residência secundária
pv-address-use-temporary = Residência temporária
pv-address-use-postal = Morada postal
pv-address-use-other = Outra
pv-nationality-mode-descent = Por filiação
pv-nationality-mode-birth-in-territory = Por nascimento no território
pv-nationality-mode-naturalisation = Por naturalização
pv-nationality-mode-marriage = Por casamento
pv-nationality-mode-registration = Por declaração
pv-nationality-mode-restoration = Por reaquisição
pv-nationality-mode-state-succession = Por mudança de soberania
pv-nationality-mode-other = Outra
pv-language-proficiency-a1 = A1 Iniciação
pv-language-proficiency-a2 = A2 Elementar
pv-language-proficiency-b1 = B1 Limiar
pv-language-proficiency-b2 = B2 Vantagem
pv-language-proficiency-c1 = C1 Autonomia
pv-language-proficiency-c2 = C2 Mestria
pv-language-proficiency-native = Língua materna
pv-isced-level-isced-0 = 0 Educação pré-escolar
pv-isced-level-isced-1 = 1 Ensino básico, 1.º e 2.º ciclos
pv-isced-level-isced-2 = 2 Ensino básico, 3.º ciclo
pv-isced-level-isced-3 = 3 Ensino secundário
pv-isced-level-isced-4 = 4 Pós-secundário não superior
pv-isced-level-isced-5 = 5 Superior de ciclo curto
pv-isced-level-isced-6 = 6 Licenciatura ou equivalente
pv-isced-level-isced-7 = 7 Mestrado ou equivalente
pv-isced-level-isced-8 = 8 Doutoramento ou equivalente
pv-income-quintile-q1 = Quinto mais baixo
pv-income-quintile-q2 = Segundo quinto
pv-income-quintile-q3 = Quinto central
pv-income-quintile-q4 = Quarto quinto
pv-income-quintile-q5 = Quinto mais alto
pv-pay-period-hourly = À hora
pv-pay-period-daily = Por dia
pv-pay-period-weekly = Por semana
pv-pay-period-monthly = Por mês
pv-pay-period-annual = Por ano
pv-tenure-owned = Propriedade
pv-tenure-co-owned = Compropriedade
pv-tenure-leasehold = Arrendamento de longa duração
pv-tenure-rented = Arrendamento
pv-tenure-usufruct = Usufruto
pv-tenure-other = Outro
pv-distinction-kind-order = Ordem
pv-distinction-kind-decoration = Condecoração
pv-distinction-kind-medal = Medalha
pv-distinction-kind-title = Título honorífico
pv-distinction-kind-other = Outra
pv-military-service-army = Exército
pv-military-service-navy = Marinha
pv-military-service-air-force = Força Aérea
pv-military-service-marines = Fuzileiros
pv-military-service-gendarmerie = Guarda nacional republicana
pv-military-service-border-guard = Guarda de fronteira
pv-military-service-national-guard = Guarda nacional
pv-military-service-other = Outro
pv-rank-category-enlisted = Praças
pv-rank-category-non-commissioned = Sargentos
pv-rank-category-warrant = Sargentos-mores ou equivalentes
pv-rank-category-officer-cadet = Cadetes
pv-rank-category-junior-officer = Oficiais subalternos e capitães
pv-rank-category-senior-officer = Oficiais superiores
pv-rank-category-general-officer = Oficiais generais
pv-iccs-section-acts-leading-to-death = 01 Atos que causam a morte
pv-iccs-section-acts-causing-harm = 02 Atos que causam dano
pv-iccs-section-sexual-acts = 03 Atos lesivos de natureza sexual
pv-iccs-section-property-with-violence = 04 Contra a propriedade com violência
pv-iccs-section-property-only = 05 Contra a propriedade
pv-iccs-section-controlled-substances = 06 Substâncias controladas
pv-iccs-section-fraud-deception-corruption = 07 Fraude, burla ou corrupção
pv-iccs-section-public-order-and-state = 08 Contra a ordem pública e o Estado
pv-iccs-section-public-safety-and-security = 09 Contra a segurança pública
pv-iccs-section-natural-environment = 10 Contra o ambiente
pv-iccs-section-other-criminal-acts = 11 Outros crimes
pv-case-outcome-convicted = Condenado
pv-case-outcome-acquitted = Absolvido
pv-case-outcome-dismissed = Arquivado
pv-case-outcome-conviction-quashed = Condenação anulada
pv-case-outcome-pardoned = Indultado
pv-case-outcome-amnestied = Amnistiado
pv-case-outcome-expunged = Cancelado do registo
pv-case-outcome-pending = Pendente
pv-case-outcome-unknown = Desconhecido
pv-religion-buddhism = Budismo
pv-religion-christianity-catholic = Cristianismo: católico
pv-religion-christianity-orthodox = Cristianismo: ortodoxo
pv-religion-christianity-protestant = Cristianismo: protestante
pv-religion-christianity-other = Cristianismo: outro
pv-religion-hinduism = Hinduísmo
pv-religion-islam-sunni = Islão: sunita
pv-religion-islam-shia = Islão: xiita
pv-religion-islam-other = Islão: outro
pv-religion-jainism = Jainismo
pv-religion-judaism = Judaísmo
pv-religion-sikhism = Siquismo
pv-religion-bahai = Fé bahá’í
pv-religion-shinto = Xintoísmo
pv-religion-taoism = Taoísmo
pv-religion-zoroastrianism = Zoroastrismo
pv-religion-traditional = Religião tradicional ou popular
pv-religion-other = Outra
pv-religion-none = Sem religião
pv-religion-unknown = Desconhecida
pv-sacrament-baptism = Batismo
pv-sacrament-confirmation = Crisma
pv-sacrament-first-communion = Primeira comunhão
pv-sacrament-reconciliation = Confissão
pv-sacrament-anointing-of-the-sick = Unção dos doentes
pv-sacrament-holy-orders = Ordem
pv-sacrament-matrimony = Matrimónio
pv-sacrament-other-rite = Outro rito
pv-political-position-far-left = Extrema-esquerda
pv-political-position-left = Esquerda
pv-political-position-centre-left = Centro-esquerda
pv-political-position-centre = Centro
pv-political-position-centre-right = Centro-direita
pv-political-position-right = Direita
pv-political-position-far-right = Extrema-direita
pv-political-position-apolitical = Apolítico
pv-political-position-other = Fora deste eixo
pv-political-position-unknown = Desconhecida
pv-membership-kind-trade-union = Sindicato
pv-membership-kind-political-party = Partido político
pv-membership-kind-professional-body = Ordem profissional
pv-membership-kind-religious-order = Ordem religiosa
pv-membership-kind-religious-association = Associação religiosa
pv-membership-kind-fraternal-order = Irmandade
pv-membership-kind-veterans-association = Associação de antigos combatentes
pv-membership-kind-sports-club = Clube desportivo
pv-membership-kind-cultural-association = Associação cultural
pv-membership-kind-charitable-association = Associação de beneficência
pv-membership-kind-other = Outra
pv-personality-instrument-neo-pi-3 = NEO-PI-3
pv-personality-instrument-neo-ffi-3 = NEO-FFI-3
pv-personality-instrument-bfi-2 = BFI-2
pv-personality-instrument-ipip-neo-120 = IPIP-NEO-120
pv-personality-instrument-tipi = TIPI
pv-personality-instrument-hexaco-pi-r = HEXACO-PI-R
pv-personality-instrument-observer-rating = Avaliação de quem a conheceu
pv-personality-instrument-inferred = Inferido dos documentos
pv-personality-instrument-other = Outro
pv-introversion-extraversion-strongly-introverted = Muito introvertido
pv-introversion-extraversion-introverted = Introvertido
pv-introversion-extraversion-ambiverted = Ambivertido
pv-introversion-extraversion-extraverted = Extrovertido
pv-introversion-extraversion-strongly-extraverted = Muito extrovertido
pv-stress-tolerance-very-low = Muito baixa
pv-stress-tolerance-low = Baixa
pv-stress-tolerance-moderate = Moderada
pv-stress-tolerance-high = Alta
pv-stress-tolerance-very-high = Muito alta
pv-decision-style-rational = Racional
pv-decision-style-intuitive = Intuitivo
pv-decision-style-dependent = Dependente
pv-decision-style-avoidant = Evitante
pv-decision-style-spontaneous = Espontâneo
pv-sport-level-recreational = Recreativo
pv-sport-level-amateur-competitive = Competição amadora
pv-sport-level-semi-professional = Semiprofissional
pv-sport-level-professional = Profissional
pv-diet-omnivore = Omnívora
pv-diet-flexitarian = Flexitariana
pv-diet-pescatarian = Pescetariana
pv-diet-vegetarian = Vegetariana
pv-diet-vegan = Vegana
pv-diet-other = Outra
pv-substance-tobacco = Tabaco e nicotina
pv-substance-alcohol = Álcool
pv-substance-cannabis = Canábis
pv-substance-opioids = Opioides
pv-substance-stimulants = Estimulantes
pv-substance-sedatives-hypnotics = Sedativos e hipnóticos
pv-substance-hallucinogens = Alucinogénios
pv-substance-inhalants = Inalantes
pv-substance-gambling = Jogo a dinheiro
pv-substance-gaming = Videojogos
pv-substance-other = Outra
pv-use-pattern-occasional-use = Consumo ocasional
pv-use-pattern-regular-use = Consumo regular
pv-use-pattern-harmful-use = Consumo nocivo
pv-use-pattern-dependence = Dependência
pv-use-pattern-in-remission = Em remissão
pv-lineage-biological = Biológica
pv-lineage-adoptive = Adotiva
pv-lineage-foster = De acolhimento
pv-lineage-step = Por afinidade
pv-lineage-guardianship = Tutela
pv-lineage-unknown = Desconhecida
pv-link-relation-godparent = Padrinho ou madrinha
pv-link-relation-godchild = Afilhado
pv-link-relation-witness = Testemunha
pv-link-relation-officiant = Celebrante
pv-link-relation-business-partner = Sócio
pv-link-relation-employer = Empregador
pv-link-relation-employee = Empregado
pv-link-relation-mentor = Mentor
pv-link-relation-apprentice = Aprendiz
pv-link-relation-close-friend = Amigo íntimo
pv-link-relation-neighbour = Vizinho
pv-link-relation-guardian = Tutor
pv-link-relation-ward = Pupilo
pv-link-relation-other = Outra
pv-country-AD = Andorra
pv-country-AE = Emirados Árabes Unidos
pv-country-AF = Afeganistão
pv-country-AG = Antígua e Barbuda
pv-country-AI = Anguila
pv-country-AL = Albânia
pv-country-AM = Arménia
pv-country-AO = Angola
pv-country-AQ = Antártida
pv-country-AR = Argentina
pv-country-AS = Samoa Americana
pv-country-AT = Áustria
pv-country-AU = Austrália
pv-country-AW = Aruba
pv-country-AX = Alanda
pv-country-AZ = Azerbaijão
pv-country-BA = Bósnia e Herzegovina
pv-country-BB = Barbados
pv-country-BD = Bangladeche
pv-country-BE = Bélgica
pv-country-BF = Burquina Faso
pv-country-BG = Bulgária
pv-country-BH = Barém
pv-country-BI = Burundi
pv-country-BJ = Benim
pv-country-BL = São Bartolomeu
pv-country-BM = Bermudas
pv-country-BN = Brunei
pv-country-BO = Bolívia
pv-country-BQ = Países Baixos Caribenhos
pv-country-BR = Brasil
pv-country-BS = Baamas
pv-country-BT = Butão
pv-country-BV = Ilha Bouvet
pv-country-BW = Botsuana
pv-country-BY = Bielorrússia
pv-country-BZ = Belize
pv-country-CA = Canadá
pv-country-CC = Ilhas dos Cocos (Keeling)
pv-country-CD = Congo-Kinshasa
pv-country-CF = República Centro-Africana
pv-country-CG = Congo-Brazzaville
pv-country-CH = Suíça
pv-country-CI = Côte d’Ivoire (Costa do Marfim)
pv-country-CK = Ilhas Cook
pv-country-CL = Chile
pv-country-CM = Camarões
pv-country-CN = China
pv-country-CO = Colômbia
pv-country-CR = Costa Rica
pv-country-CU = Cuba
pv-country-CV = Cabo Verde
pv-country-CW = Curaçau
pv-country-CX = Ilha do Natal
pv-country-CY = Chipre
pv-country-CZ = Chéquia
pv-country-DE = Alemanha
pv-country-DJ = Jibuti
pv-country-DK = Dinamarca
pv-country-DM = Domínica
pv-country-DO = República Dominicana
pv-country-DZ = Argélia
pv-country-EC = Equador
pv-country-EE = Estónia
pv-country-EG = Egito
pv-country-EH = Sara Ocidental
pv-country-ER = Eritreia
pv-country-ES = Espanha
pv-country-ET = Etiópia
pv-country-FI = Finlândia
pv-country-FJ = Fiji
pv-country-FK = Ilhas Falkland
pv-country-FM = Micronésia
pv-country-FO = Ilhas Faroé
pv-country-FR = França
pv-country-GA = Gabão
pv-country-GB = GB
pv-country-GD = Granada
pv-country-GE = Geórgia
pv-country-GF = Guiana Francesa
pv-country-GG = Guernesey
pv-country-GH = Gana
pv-country-GI = Gibraltar
pv-country-GL = Gronelândia
pv-country-GM = Gâmbia
pv-country-GN = Guiné
pv-country-GP = Guadalupe
pv-country-GQ = Guiné Equatorial
pv-country-GR = Grécia
pv-country-GS = Ilhas Geórgia do Sul e Sandwich do Sul
pv-country-GT = Guatemala
pv-country-GU = Guame
pv-country-GW = Guiné-Bissau
pv-country-GY = Guiana
pv-country-HK = Hong Kong, RAE da China
pv-country-HM = Ilhas Heard e McDonald
pv-country-HN = Honduras
pv-country-HR = Croácia
pv-country-HT = Haiti
pv-country-HU = Hungria
pv-country-ID = Indonésia
pv-country-IE = Irlanda
pv-country-IL = Israel
pv-country-IM = Ilha de Man
pv-country-IN = Índia
pv-country-IO = Território Britânico do Oceano Índico
pv-country-IQ = Iraque
pv-country-IR = Irão
pv-country-IS = Islândia
pv-country-IT = Itália
pv-country-JE = Jersey
pv-country-JM = Jamaica
pv-country-JO = Jordânia
pv-country-JP = Japão
pv-country-KE = Quénia
pv-country-KG = Quirguistão
pv-country-KH = Camboja
pv-country-KI = Quiribáti
pv-country-KM = Comores
pv-country-KN = São Cristóvão e Neves
pv-country-KP = Coreia do Norte
pv-country-KR = Coreia do Sul
pv-country-KW = Koweit
pv-country-KY = Ilhas Caimão
pv-country-KZ = Cazaquistão
pv-country-LA = Laos
pv-country-LB = Líbano
pv-country-LC = Santa Lúcia
pv-country-LI = Listenstaine
pv-country-LK = Sri Lanca
pv-country-LR = Libéria
pv-country-LS = Lesoto
pv-country-LT = Lituânia
pv-country-LU = Luxemburgo
pv-country-LV = Letónia
pv-country-LY = Líbia
pv-country-MA = Marrocos
pv-country-MC = Mónaco
pv-country-MD = Moldávia
pv-country-ME = Montenegro
pv-country-MF = São Martinho
pv-country-MG = Madagáscar
pv-country-MH = Ilhas Marshall
pv-country-MK = Macedónia do Norte
pv-country-ML = Mali
pv-country-MM = Mianmar (Birmânia)
pv-country-MN = Mongólia
pv-country-MO = Macau, RAE da China
pv-country-MP = Ilhas Marianas do Norte
pv-country-MQ = Martinica
pv-country-MR = Mauritânia
pv-country-MS = Monserrate
pv-country-MT = Malta
pv-country-MU = Maurícia
pv-country-MV = Maldivas
pv-country-MW = Maláui
pv-country-MX = México
pv-country-MY = Malásia
pv-country-MZ = Moçambique
pv-country-NA = Namíbia
pv-country-NC = Nova Caledónia
pv-country-NE = Níger
pv-country-NF = Ilha Norfolk
pv-country-NG = Nigéria
pv-country-NI = Nicarágua
pv-country-NL = Países Baixos
pv-country-NO = Noruega
pv-country-NP = Nepal
pv-country-NR = Nauru
pv-country-NU = Niuê
pv-country-NZ = Nova Zelândia
pv-country-OM = Omã
pv-country-PA = Panamá
pv-country-PE = Peru
pv-country-PF = Polinésia Francesa
pv-country-PG = Papua-Nova Guiné
pv-country-PH = Filipinas
pv-country-PK = Paquistão
pv-country-PL = Polónia
pv-country-PM = São Pedro e Miquelão
pv-country-PN = Ilhas Pitcairn
pv-country-PR = Porto Rico
pv-country-PS = Territórios palestinianos
pv-country-PT = Portugal
pv-country-PW = Palau
pv-country-PY = Paraguai
pv-country-QA = Catar
pv-country-RE = Reunião
pv-country-RO = Roménia
pv-country-RS = Sérvia
pv-country-RU = Rússia
pv-country-RW = Ruanda
pv-country-SA = Arábia Saudita
pv-country-SB = Ilhas Salomão
pv-country-SC = Seicheles
pv-country-SD = Sudão
pv-country-SE = Suécia
pv-country-SG = Singapura
pv-country-SH = Santa Helena
pv-country-SI = Eslovénia
pv-country-SJ = Svalbard e Jan Mayen
pv-country-SK = Eslováquia
pv-country-SL = Serra Leoa
pv-country-SM = São Marinho
pv-country-SN = Senegal
pv-country-SO = Somália
pv-country-SR = Suriname
pv-country-SS = Sudão do Sul
pv-country-ST = São Tomé e Príncipe
pv-country-SV = Salvador
pv-country-SX = São Martinho (Sint Maarten)
pv-country-SY = Síria
pv-country-SZ = Essuatíni
pv-country-TC = Ilhas Turcas e Caicos
pv-country-TD = Chade
pv-country-TF = Territórios Austrais Franceses
pv-country-TG = Togo
pv-country-TH = Tailândia
pv-country-TJ = Tajiquistão
pv-country-TK = Toquelau
pv-country-TL = Timor-Leste
pv-country-TM = Turquemenistão
pv-country-TN = Tunísia
pv-country-TO = Tonga
pv-country-TR = Turquia
pv-country-TT = Trindade e Tobago
pv-country-TV = Tuvalu
pv-country-TW = Taiwan
pv-country-TZ = Tanzânia
pv-country-UA = Ucrânia
pv-country-UG = Uganda
pv-country-UM = Ilhas Menores Afastadas dos EUA
pv-country-US = Estados Unidos
pv-country-UY = Uruguai
pv-country-UZ = Usbequistão
pv-country-VA = Cidade do Vaticano
pv-country-VC = São Vicente e Granadinas
pv-country-VE = Venezuela
pv-country-VG = Ilhas Virgens Britânicas
pv-country-VI = Ilhas Virgens dos EUA
pv-country-VN = Vietname
pv-country-VU = Vanuatu
pv-country-WF = Wallis e Futuna
pv-country-WS = Samoa
pv-country-YE = Iémen
pv-country-YT = Maiote
pv-country-ZA = África do Sul
pv-country-ZM = Zâmbia
pv-country-ZW = Zimbabué
pv-country-SU = União Soviética
pv-country-DD = Alemanha Oriental
pv-country-YU = Jugoslávia
pv-country-CS = Checoslováquia
pv-country-OT = Império Otomano
lang-aa = Afar
lang-ab = Abcázio
lang-ae = Avéstico
lang-af = Africanês
lang-ak = Akan
lang-am = Amárico
lang-an = Aragonês
lang-ar = Árabe
lang-as = Assamês
lang-av = Avaric
lang-ay = Aimará
lang-az = Azeri
lang-ba = Bashkir
lang-be = Bielorrusso
lang-bg = Búlgaro
lang-bi = Bislamá
lang-bm = Bambara
lang-bn = Bengalês
lang-bo = Tibetano
lang-br = Bretão
lang-bs = Bósnio
lang-ca = Catalão
lang-ce = Checheno
lang-ch = Chamorro
lang-co = Córsico
lang-cr = Cree
lang-cs = Checo
lang-cu = Eslavo eclesiástico
lang-cv = Chuvash
lang-cy = Galês
lang-da = Dinamarquês
lang-de = Alemão
lang-dv = Divehi
lang-dz = Dzonga
lang-ee = Ewe
lang-el = Grego
lang-en = Inglês
lang-eo = Esperanto
lang-es = Espanhol
lang-et = Estónio
lang-eu = Basco
lang-fa = Persa
lang-ff = Fula
lang-fi = Finlandês
lang-fj = Fijiano
lang-fo = Feroês
lang-fr = Francês
lang-fy = Frísico ocidental
lang-ga = Irlandês
lang-gd = Gaélico escocês
lang-gl = Galego
lang-gn = Guarani
lang-gu = Guzerate
lang-gv = Manx
lang-ha = Haúça
lang-he = Hebraico
lang-hi = Hindi
lang-ho = Hiri motu
lang-hr = Croata
lang-ht = Haitiano
lang-hu = Húngaro
lang-hy = Arménio
lang-hz = Herero
lang-ia = Interlíngua
lang-id = Indonésio
lang-ie = Interlingue
lang-ig = Igbo
lang-ii = Sichuan yi
lang-ik = Inupiaque
lang-io = Ido
lang-is = Islandês
lang-it = Italiano
lang-iu = Inuktitut
lang-ja = Japonês
lang-jv = Javanês
lang-ka = Georgiano
lang-kg = Congolês
lang-ki = Quicuio
lang-kj = Cuanhama
lang-kk = Cazaque
lang-kl = Gronelandês
lang-km = Khmer
lang-kn = Canarim
lang-ko = Coreano
lang-kr = Canúri
lang-ks = Caxemira
lang-ku = Curdo
lang-kv = Komi
lang-kw = Córnico
lang-ky = Quirguiz
lang-la = Latim
lang-lb = Luxemburguês
lang-lg = Ganda
lang-li = Limburguês
lang-ln = Lingala
lang-lo = Laosiano
lang-lt = Lituano
lang-lu = Luba-catanga
lang-lv = Letão
lang-mg = Malgaxe
lang-mh = Marshalês
lang-mi = Maori
lang-mk = Macedónio
lang-ml = Malaiala
lang-mn = Mongol
lang-mr = Marata
lang-ms = Malaio
lang-mt = Maltês
lang-my = Birmanês
lang-na = Nauruano
lang-nb = Norueguês bokmål
lang-nd = Ndebele do norte
lang-ne = Nepalês
lang-ng = Dongo
lang-nl = Neerlandês
lang-nn = Norueguês nynorsk
lang-no = Norueguês
lang-nr = Ndebele do sul
lang-nv = Navajo
lang-ny = Nianja
lang-oc = Occitano
lang-oj = Ojibwa
lang-om = Oromo
lang-or = Oriá
lang-os = Ossético
lang-pa = Panjabi
lang-pi = Páli
lang-pl = Polaco
lang-ps = Pastó
lang-pt = Português
lang-qu = Quíchua
lang-rm = Romanche
lang-rn = Rundi
lang-ro = Romeno
lang-ru = Russo
lang-rw = Quiniaruanda
lang-sa = Sânscrito
lang-sc = Sardo
lang-sd = Sindi
lang-se = Sami do norte
lang-sg = Sango
lang-sh = Servo-croata
lang-si = Cingalês
lang-sk = Eslovaco
lang-sl = Esloveno
lang-sm = Samoano
lang-sn = Shona
lang-so = Somali
lang-sq = Albanês
lang-sr = Sérvio
lang-ss = Suázi
lang-st = Sesoto
lang-su = Sundanês
lang-sv = Sueco
lang-sw = Suaíli
lang-ta = Tâmil
lang-te = Telugu
lang-tg = Tajique
lang-th = Tailandês
lang-ti = Tigrínia
lang-tk = Turcomano
lang-tl = Tagalo
lang-tn = Tswana
lang-to = Tonga
lang-tr = Turco
lang-ts = Tsonga
lang-tt = Tatar
lang-tw = Twi
lang-ty = Taitiano
lang-ug = Uigur
lang-uk = Ucraniano
lang-ur = Urdu
lang-uz = Usbeque
lang-ve = Venda
lang-vi = Vietnamita
lang-vo = Volapuque
lang-wa = Valão
lang-wo = Uólofe
lang-xh = Xosa
lang-yi = Iídiche
lang-yo = Ioruba
lang-za = Zhuang
lang-zh = Chinês mandarim
lang-zu = Zulu
person-tab-profile = Perfil
profile-groups-label = Secções do perfil
profile-group-withheld = Parte desta secção não está visível para si
profile-withheld = Registado para esta pessoa e não visível para si: { $classes }.
profile-empty = Ainda nada está registado nesta secção.
profile-earlier = formulário anterior
profile-earlier-title = Registado por uma versão anterior desta aplicação, num campo que o AXGF 1.1 não prevê. Mantém-se tal como foi escrito.
profile-other-names = { $n ->
        [one] e mais um nome
       *[other] e mais { $n } nomes
    }
profile-edit-group = Editar «{ $group }»
profile-summary-link = { $n ->
        [one] Um dado no perfil
       *[other] { $n } dados no perfil
    }
profile-from = desde
profile-until = até
profile-yes = Sim
profile-no = Não
profile-value = Valor
profile-editor-title = Perfil
profile-problems = Parte do que foi introduzido não pôde ser guardado. Cada problema está indicado junto ao seu campo, e nada foi escrito.
profile-editor-withheld = Esta secção contém também, para esta pessoa, dados da categoria { $classes } que não pode ler. Não são mostrados aqui, e guardar este formulário deixa-os como estão.
profile-living-class-note = Esta pessoa está registada como viva. O que introduzir aqui numa categoria sensível só será visto pelos administradores.
profile-relationships-elsewhere = Pais, cônjuges, filhos, padrinhos e testemunhas não estão guardados nesta pessoa. São famílias, ligações e eventos que a mencionam — por isso cada alteração aqui muda também o registo de todas as outras pessoas envolvidas.
profile-documents-first = Um artefacto remete para um documento associado a esta pessoa. Anexe primeiro o ficheiro.
profile-editor-nothing = Não há nada nesta secção que possa editar.
profile-new-entry = Nova entrada
profile-provenance = Data, fonte e fiabilidade
profile-from-date = Válido desde
profile-until-date = Válido até
profile-remove-entry = Remover esta entrada
profile-add-entry = Adicionar outra entrada
profile-no-such-group-title = Secção inexistente
profile-no-such-group-detail = O perfil não tem nenhuma secção com esse nome.
profile-error-number = Um valor deste campo tem de ser um número.
profile-error-integer = Um valor deste campo tem de ser um número inteiro.
profile-error-range = Um número está fora do intervalo permitido para este atributo.
profile-error-term = Um valor não está entre as opções disponíveis.
profile-error-required = Falta um campo obrigatório a uma entrada.
profile-error-one-of = Uma entrada precisa de pelo menos um dos seus campos principais.
profile-error-confidence = A fiabilidade vai de 0 a 1, por exemplo 0,8.
profile-error-time = Uma hora escreve-se em horas e minutos, por exemplo 05:40.
profile-error-currency = Uma moeda escreve-se com o seu código de três letras, por exemplo EUR.
profile-error-language = Uma língua escreve-se com o seu código, por exemplo pt ou zh-Hans.
profile-error-coordinates = As coordenadas precisam de uma latitude entre −90 e 90 e de uma longitude entre −180 e 180.
profile-error-rank-country = O posto pertence a um país diferente do escolhido.
record-unknown-place = [Local desconhecido]
record-missing-document = [Documento em falta]

## Interface

confidence-certain = Certeza { $percent } % — praticamente certo
confidence-high = Certeza { $percent } % — bem fundamentado
confidence-medium = Certeza { $percent } % — plausível mas por confirmar
confidence-low = Certeza { $percent } % — conjetura
tree-edge-union-between = { $from } e { $to } — { $confidence }
tree-edge-parentage-of = { $from }, progenitor de { $to } — { $confidence }
record-note-biography = Biografia
record-note-birth-date-as-recorded = Data de nascimento, tal como registada
record-note-death-date-as-recorded = Data de óbito, tal como registada
record-note-event-date-as-recorded = Data de «{ $event }», tal como registada
record-unknown-source = [Fonte desconhecida]
record-untitled-source = [Fonte sem título]
record-unnamed = [Sem nome]
record-untitled = [Sem título]
record-period-from = desde { $date }
record-period-until = até { $date }
record-dates-unrecorded = datas por registar
record-link-unlabelled = ligado a
record-link-reverse = { $label } (de)
record-place-worked-as = Trabalhou como { $title }
record-place-married-to = Casamento com { $name }
record-place-married = Casamento
record-source-use-name = o nome «{ $name }»
record-source-use-working-as = o ofício de { $title }
record-source-use-union-with = a união com { $name }
record-source-use-union = a união
record-lifespan-born = n. { $year }
record-lifespan-died = f. { $year }
size-bytes = { $n ->
        [one] { $n } byte
       *[other] { $n } bytes
    }
size-kb = { $n } KB
size-mb = { $n } MB
size-gb = { $n } GB
calendar-gregorian = gregoriano
calendar-julian = juliano
calendar-hebrew = hebraico
calendar-hijri = islâmico
calendar-persian = persa
calendar-chinese = chinês
calendar-ethiopian = etíope
calendar-japanese_era = eras japonesas
calendar-republican_french = republicano francês
calendar-roman = romano
diff-summary-none = não alterou nenhum campo
diff-summary-one = alterou { $a }
diff-summary-two = alterou { $a } e { $b }
diff-summary-many = alterou { $a }, { $b } e { $n ->
        [one] mais um campo
       *[other] mais { $n } campos
    }
diff-saved-none = nenhum campo alterado
diff-saved-one = { $a } alterado
diff-saved-two = { $a } e { $b } alterados
diff-saved-many = { $a }, { $b } e { $n ->
        [one] mais um campo alterados
       *[other] mais { $n } campos alterados
    }
history-created = criou
history-deleted = eliminou
history-attached = anexou um ficheiro
admin-raw-json-unparsed = Não foi possível ler o JSON em bruto ({ $error }). Nada foi guardado.
conflict-someone = Alguém
conflict-unrecorded-time = (hora não registada)
dedup-merged-persons = { $n ->
        [one] uma pessoa fundida
       *[other] { $n } pessoas fundidas
    }
dedup-merged-families = { $n ->
        [one] uma família fundida
       *[other] { $n } famílias fundidas
    }
dedup-manual-review = { $n ->
        [one] um caso deixado para revisão por uma pessoa
       *[other] { $n } casos deixados para revisão por uma pessoa
    }
dedup-nothing = Nada a assinalar.
validate-errors = { $n ->
        [one] um erro
       *[other] { $n } erros
    }
validate-warnings = { $n ->
        [one] um aviso
       *[other] { $n } avisos
    }
validate-notes = { $n ->
        [one] uma nota
       *[other] { $n } notas
    }
validate-nothing = Nada a assinalar.
list-separator = { ", " }
result-written = O arquivo foi escrito no disco.
result-refused = A biblioteca recusou esta operação. O arquivo no disco não foi alterado.
convert-error-no-file = Não foi enviado nenhum ficheiro. Escolha primeiro um ficheiro .ged.
convert-error-file-too-large = Este ficheiro tem { $size } MB e o limite é de { $limit } MB. Nada foi convertido.
convert-error-too-large = O envio ultrapassa o limite de { $limit } MB. Nada foi convertido.
convert-error-unreadable = Não foi possível ler o envio ({ $error }). Nada foi convertido.
convert-error-not-gedcom = Isto não parece um ficheiro GEDCOM: um ficheiro GEDCOM 5.5.1 começa com uma linha «0 HEAD». Nada foi convertido.
convert-error-packaging = O ficheiro foi convertido mas não foi possível empacotá-lo ({ $error }).
completeness-fraction = { $part } de { $whole }
event-category-adoption = Adoção
event-category-migration = Migração
event-category-naturalization = Naturalização
event-category-incarceration = Prisão
event-category-name_change = Mudança de nome
event-category-legal = Processo judicial
event-category-religious = Facto religioso
event-category-social = Facto social
event-category-historical = Facto histórico
precision-quarter_century = ao quarto de século
source-type-birth_certificate = assento de nascimento
source-type-death_certificate = assento de óbito
source-type-marriage_certificate = assento de casamento
source-type-census = recenseamento
source-type-baptism_record = assento de batismo
source-type-burial_record = registo de sepultura
source-type-will = testamento
source-type-land_record = registo predial
source-type-military_record = registo militar
source-type-immigration_record = registo de imigração
source-type-naturalization = processo de naturalização
source-type-passport = passaporte
source-type-photograph = fotografia
source-type-letter = carta
source-type-diary = diário
source-type-newspaper = jornal
source-type-oral_tradition = tradição oral
source-type-dna = teste de ADN
source-type-family_bible = Bíblia de família
source-type-gravestone = lápide
source-type-published_genealogy = genealogia publicada
source-type-other = outra fonte
source-status-verified = conferida com o original
source-status-unverified = ainda não conferida
source-status-lost = perdida
source-status-known_missing = sabe-se que falta
document-type-birth_certificate = assento de nascimento
document-type-death_certificate = assento de óbito
document-type-marriage_certificate = assento de casamento
document-type-census_page = folha de recenseamento
document-type-baptism_record = assento de batismo
document-type-military_record = registo militar
document-type-will = testamento
document-type-land_record = registo predial
document-type-diary = diário
document-type-newspaper_clipping = recorte de jornal
document-type-gravestone_photo = fotografia de uma lápide
document-type-family_tree_drawing = árvore genealógica desenhada
document-type-audio = gravação de som
document-type-video = gravação de vídeo
document-status-present = guardado aqui
document-status-referenced = citado, guardado noutro sítio
document-status-known_missing = sabe-se que falta
document-status-lost = perdido
document-status-unknown = paradeiro desconhecido
diag-unsupported_spec_version = O arquivo declara uma versão do AXGF que este programa não sabe ler.
diag-invalid_json = Algo que devia ser JSON não pode ser lido.
diag-invalid_bundle_structure = O arquivo não está organizado como o AXGF exige.
diag-schema_validation_failed = Um registo não corresponde ao esquema do AXGF.
diag-dangling_reference = Um registo aponta para outro que não está no arquivo.
diag-duplicate_entity_id = Dois registos partilham o mesmo identificador.
diag-duplicate_unique_ref = Dois registos reclamam a mesma referência, que devia ser única.
diag-cycle_detected = As ligações familiares andam em círculo: alguém seria o seu próprio antepassado.
diag-chronology_conflict = As datas contradizem-se, por exemplo um filho nascido antes de um progenitor.
diag-out_of_vocabulary = Um valor não é um dos termos que a sua lista admite.
diag-claim_inconsistent = Uma afirmação contradiz-se a si mesma ou a outra afirmação sobre a mesma coisa.
diag-spec_version_mismatch = A versão do AXGF declarada por um registo não corresponde ao que ele contém.
diag-unknown_attribute = Um registo tem um atributo que o AXGF não define.
diag-entity_not_found = O registo a alterar não está no arquivo.
diag-entity_already_exists = Já existe um registo com este identificador.
diag-unknown_entity_kind = Isto não é um tipo de registo que o AXGF tenha.
diag-delete_blocked_by_reference = O registo não pode ser eliminado enquanto outros apontarem para ele.
diag-manual_review_required = Uma pessoa tem de ver isto; não foi alterado automaticamente.
diag-zip_read_error = Não foi possível ler o ficheiro do arquivo.
diag-zip_write_error = Não foi possível escrever o ficheiro do arquivo.
diag-payloads_external = Os ficheiros anexos estão guardados fora dos dados do arquivo.
diag-payload_source_failed = Não foi possível ler um ficheiro anexo.
diag-payload_sink_failed = Não foi possível escrever um ficheiro anexo.
diag-gedcom_parse_error = Não foi possível compreender uma linha do ficheiro GEDCOM.
diag-gedcom_unrecognized_tag = O ficheiro GEDCOM usa uma etiqueta que a importação não conhece, por isso essa entrada não foi trazida.
diag-internal = Algo correu mal dentro da biblioteca.
field-person-display-name = Nome apresentado
field-person-display-name-hint = O nome mostrado em todo o sítio.
field-person-gender = Género
field-person-living = Vivo/a
field-person-birth-date = Data de nascimento
field-date-value-hint = Um ano, ano e mês ou uma data completa: 1923, 1923-04 ou 1923-04-12. Deixe em branco se ninguém souber.
field-person-birth-precision = Precisão do nascimento
field-precision-hint = Com que precisão a fonte o estabelece.
field-person-birth-circa = Nascimento aproximado
field-circa-hint = Mostrado como «cerca de 1923» e não como afirmação exata.
field-person-birth-place = Identificador do local de nascimento
field-person-birth-confidence = Certeza do nascimento
field-person-confidence-hint = Quão certo está. É isto que o sítio desenha.
field-person-death-date = Data de óbito
field-person-death-precision = Precisão do óbito
field-person-death-circa = Óbito aproximado
field-person-death-place = Identificador do local de óbito
field-person-death-confidence = Certeza do óbito
field-person-death-cause = Causa de morte
field-person-bio = Biografia
field-notes = Notas
field-family-name = Nome da família
field-description = Descrição
field-family-union-type = Tipo de união
field-family-union-status = Estado da união
field-family-union-confidence = Certeza da união
field-family-union-confidence-hint = Define a intensidade com que a linha entre o casal é desenhada na árvore.
field-family-union-start = Início da união
field-family-union-end = Fim da união
field-family-notes-hint = O casal e os filhos são listas: edite-os no JSON em bruto abaixo ou na página de relações da pessoa.
field-category = Categoria
field-required-hint = Obrigatório.
field-event-subcategory = Subcategoria
field-date = Data
field-event-date-hint = Exigida pelo esquema.
field-precision = Precisão
field-circa = Aproximada
field-place-id = Identificador do local
field-confidence = Certeza
field-source-id = Identificador da fonte
field-link-from-type = Tipo de origem
field-link-from-id = Identificador de origem
field-link-to-type = Tipo de destino
field-link-to-id = Identificador de destino
field-link-label = Nome da ligação
field-link-label-hint = Lê-se no sentido da ligação: «padrinho», «patrão», «testemunha». Obrigatório.
field-link-label-reverse = Nome no sentido inverso
field-link-label-reverse-hint = Como se lê a partir da outra ponta: «afilhado», «empregado».
field-link-bidirectional = Lê-se igual nos dois sentidos
field-valid-from = Válido desde
field-link-valid-from-hint = Quando começou a relação.
field-valid-until = Válido até
field-link-confidence-hint = «Certeza de 85 %, segundo uma carta de família» — aquilo que o GEDCOM não consegue dizer.
field-note = Nota
field-occupation-person-id = Identificador da pessoa
field-occupation-title = Ofício
field-occupation-title-hint = Obrigatório, por exemplo Professora primária.
field-occupation-title-latin = Ofício (alfabeto latino)
field-occupation-employer = Patrão
field-occupation-from = Desde
field-occupation-from-hint = Um ofício é um período. Indicar os dois extremos é o que permite desenhá-lo como uma barra.
field-occupation-until = Até
field-source-title = Título
field-source-type = Tipo de fonte
field-source-reliability = Fiabilidade
field-source-reliability-hint = Obrigatório. Mostrada como distintivo junto de cada facto que assenta nesta fonte.
field-source-status = Estado da fonte
field-source-repository = Local de guarda
field-source-repository-reference = Cota no local de guarda
field-source-transcription = Transcrição
field-place-name = Nome principal
field-place-name-lang = Língua do nome
field-place-name-lang-hint = Um código de língua, por exemplo en, fr ou pl.
field-place-type = Tipo de local
field-place-region = Região
field-place-country-current = País atual
field-place-country-current-hint = A história das suas fronteiras é uma lista: edite-a no JSON em bruto abaixo.
field-document-filename = Nome do ficheiro
field-document-mime-type = Tipo de conteúdo
field-document-mime-type-hint = Obrigatório, por exemplo image/jpeg.
field-document-type = Tipo de documento
field-document-status = Estado do ficheiro
field-document-url = Endereço web
field-document-caption = Legenda
lang-zh-Hans = Chinês simplificado
