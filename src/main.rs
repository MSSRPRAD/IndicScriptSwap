use crate::read_mappings::ScriptData;
use read_mappings::read_mappings;
mod convert;
mod functions;
mod read_mappings;

fn main() {
    let foo = ScriptData {
        data: read_mappings(),
    };

    let devanagari = foo.data.scripts.get("devanagari").unwrap();
    let slp1 = foo.data.scripts.get("slp1").unwrap();

    let mut input: String = "asty uttarasyAM diSi devatAtmA himAlayo nAma nagADirAjaH .
    pUrvAparO toyaniDI vigAhya sTitaH pfTivyA iva mAnadaRqaH ..
    yaM sarvaSElAH parikalpya vatsaM merO sTite dogDari dohadakze .
    BAsvanti ratnAni mahOzaDIS ca pfTUpadizwAM duduhur DaritrIm ..
    anantaratnapraBavasya yasya himaM na sOBAgyavilopi jAtam .
    eko hi dozo guRasaMnipAte nimajjatIndoH kiraRezv ivANkaH ..
    yaS cApsaroviBramamaRqanAnAM saMpAdayitrIM SiKarEr biBarti .
    balAhakacCedaviBaktarAgAm akAlasaMDyAm iva DAtumattAm ..
    AmeKalaM saMcaratAM GanAnAM cCAyAm aDaHsAnugatAM nizevya .
    udvejitA vfzwiBir ASrayante SfNgARi yasyAtapavanti sidDAH ..
    padaM tuzArasrutiDOtaraktaM yasminn adfzwvApi hatadvipAnAm .
    vidanti mArgaM naKaranDramuktEr muktAPalEH kesariRAM kirAtAH ..
    nyastAkzarA DAturasena yatra BUrjatvacaH kuYjarabinduSoRAH .
    vrajanti vidyADarasundarIRAm anaNgaleKakriyayopayogam ..
    yaH pUrayan kIcakaranDraBAgAn darImuKotTena samIraRena .
    udgAsyatAm icCati kiMnarARAM tAnapradAyitvam ivopagantum ..
    kapolakaRqUH kariBir vinetuM viGawwitAnAM saraladrumARAm .
    yatra srutakzIratayA prasUtaH sAnUni ganDaH suraBIkaroti ..
    vanecarARAM vanitAsaKAnAM darIgfhotsaNganizaktaBAsaH .
    Bavanti yatrOzaDayo rajanyAm atElapUrAH suratapradIpAH ..
    udvejayaty aNgulipArzRiBAgAn mArge SilIBUtahime 'pi yatra .
    na durvahaSroRipayoDarArtA Bindanti mandAM gatim aSvamuKyaH ..
    divAkarAd rakzati yo guhAsu lInaM divA BItam ivAnDakAram .
    kzudre 'pi nUnaM SaraRaM prapanne mamatvam uccEHSirasAM satIva ..
    lANgUlavikzepavisarpiSoBEr itas tataS candramarIcigOrEH .
    yasyArTayuktaM girirAjaSabdaM kurvanti vAlavyajanES camaryaH ..
    yatrAMSukAkzepavilajjitAnAM yadfcCayA kiMpuruzANganAnAm .
    darIgfhadvAravilambibimbAs tiraskariRyo jaladA Bavanti ..
    BAgIraTInirJarasIkarARAM voQA muhuH kampitadevadAruH .
    yad vAyur anvizwamfgEH kirAtEr Asevyate BinnaSiKaRqibarhaH ..
    saptarzihastAvacitAvaSezARy aDo vivasvAn parivartamAnaH .
    padmAni yasyAgrasaroruhARi praboDayaty UrDvamuKEr mayUKEH ..
    yajYANgayonitvam avekzya yasya sAraM DaritrIDaraRakzamaM ca .
    prajApatiH kalpitayajYaBAgaM SElADipatyaM svayam anvatizWat ..
    sa mAnasIM merusaKaH pitFRAM kanyAM kulasya sTitaye sTitijYaH .
    menAM munInAm api mAnanIyAm AtmAnurUpAM viDinopayeme ..
    kAlakrameRATa tayoH pravftte svarUpayogye surataprasaNge .
    manoramaM yOvanam udvahantyA garBo 'Bavad BUDararAjapatnyAH ..
    asUta sA nAgavaDUpaBogyaM mEnAkam amBoniDibadDasaKyam .
    krudDe 'pi pakzacCidi vftraSatrAv avedanAjYaM kuliSakzatAnAm ..
    aTAvamAnena pituH prayuktA dakzasya kanyA BavapUrvapatnI .
    satI satI yogavisfzwadehA tAM janmane SElavaDUM prapede ..
    sA BUDarARAm aDipena tasyAM samADimatyAm udapAdi BavyA .
    samyakprayogAd aparikzatAyAM nItAv ivotsAhaguRena saMpat ..
    prasannadik pAMsuviviktavAtaM SaNKasvanAnantarapuzpavfzwi .
    SarIriRAM sTAvarajaNgamAnAM suKAya tajjanmadinaM baBUva ..
    tayA duhitrA sutarAM savitrI sPuratpraBAmaRqalayA cakAse .
    vidUraBUmir navameGaSabdAd udBinnayA ratnaSalAkayeva ..
    dine dine sA parivarDamAnA labDodayA cAndramasIva leKA .
    pupoza lAvaRyamayAn viSezAY jyotsnAntarARIva kalAntarARi ..
    tAM pArvatIty ABijanena nAmnA banDupriyAM banDujano juhAva .
    u meti mAtrA tapaso nizidDA paScAd umAKyAM sumuKI jagAma ..
    mahIBftaH putravato 'pi dfzwis tasminn apatye na jagAma tfptim .
    anantapuzpasya maDor hi cUte dvirePamAlA saviSezasaNgA ..
    praBAmahatyA SiKayeva dIpas trimArgayeva tridivasya mArgaH .
    saMskAravatyeva girA manIzI tayA sa pUtaS ca viBUzitaS ca ..
    mandAkinIsEkatavedikABiH sA kandukEH kftrimaputrakES ca .
    reme muhur maDyagatA saKInAM krIqArasaM nirviSatIva bAlye ..
    tAM haMsamAlAH SaradIva gaNgAM mahOzaDiM naktam ivAtmaBAsaH .
    sTiropadeSAm upadeSakAle prapedire prAktanajanmavidyAH ..
    asaMBftaM maRqanam aNgayazwer anAsavAKyaM karaRaM madasya .
    kAmasya puzpavyatiriktam astraM bAlyAt paraM sATa vayaH prapede ..
    unmIlitaM tUlikayeva citraM sUryAMSuBir Binnam ivAravindam .
    baBUva tasyAS caturasraSoBi vapur viBaktaM navayOvanena ..
    aByunnatANguzWanaKapraBABir nikzepaRAd rAgam ivodgirantO .
    Ajahratus taccaraRO pfTivyAM sTalAravindaSriyam avyavasTAm ..
    sA rAjahaMsEr iva saMnatANgI gatezu lIlAYcitavikramezu .
    vyanIyata pratyupadeSalubDEr AditsuBir nUpurasiYjitAni ..
    vfttAnupUrve ca na cAtidIrGe jaNGe SuBe sfzwavatas tadIye .
    SezANganirmARaviDO viDAtur lAvaRya utpAdya ivAsa yatnaH ..
    nAgendrahastAs tvaci karkaSatvAd ekAntaSEtyAt kadalIviSezAH .
    labDvApi loke pariRAhi rUpaM jAtAs tadUrvor upamAnabAhyAH ..
    etAvatA nanv anumeyaSoBaM kAYcIguRasTAnam aninditAyAH .
    AropitaM yad giriSena paScAd ananyanArIkamanIyam aNkam ..
    tasyAH pravizwA natanABiranDraM rarAja tanvI navalomarAjiH .
    nIvIm atikramya sitetarasya tanmeKalAmaDyamaRer ivArciH ..
    maDyena sA vedivilagnamaDyA valitrayaM cAru baBAra bAlA .
    ArohaRArTaM navayOvanena kAmasya sopAnam iva prayuktam ..
    anyonyam utpIqayad utpalAkzyAH stanadvayaM pARqu taTA pravfdDam .
    maDye yaTA SyAmamuKasya tasya mfRAlasUtrAntaram apy alaByam ..
    SirIzapuzpADikasOkumAryO bAhU tadIyAv iti me vitarkaH .
    parAjitenApi kftO harasya yO kaRWapASO makaraDvajena ..
    kaRWasya tasyAH stanabanDurasya muktAkalApasya ca nistalasya .
    anyonyaSoBAjananAd baBUva sADAraRo BUzaRaBUzyaBAvaH ..
    candraM gatA padmaguRAn na BuNkte padmASritA cAndramasIm aBiKyAm .
    umAmuKaM tu pratipadya lolA dvisaMSrayAM prItim avApa lakzmIH ..
    puzpaM pravAlopahitaM yadi syAn muktAPalaM vA sPuwavidrumasTam .
    tato 'nukuryAd viSadasya tasyAs tAmrOzWaparyastarucaH smitasya ..
    svareRa tasyAm amftasruteva prajalpitAyAm aBijAtavAci .
    apy anyapuzwA pratikUlaSabdA Srotur vitantrIr iva tAqyamAnA ..
    pravAtanIlotpalanirviSezam aDIraviprekzitam AyatAkzyA .
    tayA gfhItaM nu mfgANganAByas tato gfhItaM nu mfgANganABiH ..
    tasyAH SalAkAYjananirmiteva kAntir Bruvor AnataleKayor yA .
    tAM vIkzya lIlAcaturAm anaNgaH svacApasOndaryamadaM mumoca ..
    lajjA tiraScAM yadi cetasi syAd asaMSayaM parvatarAjaputryAH .
    taM keSapASaM prasamIkzya kuryur vAlapriyatvaM SiTilaM camaryaH ..
    sarvopamAdravyasamuccayena yaTApradeSaM viniveSitena .
    sA nirmitA viSvasfjA prayatnAd ekasTasOndaryadidfkzayeva ..
    tAM nAradaH kAmacaraH kadA cit kanyAM kila prekzya pituH samIpe .
    samAdideSEkavaDUM BavitrIM premRA SarIrArDaharAM harasya ..
    guruH pragalBe 'pi vayasy ato 'syAs tasTO nivfttAnyavarABilAzaH .
    fte kfSAnor na hi mantrapUtam arhanti tejAMsy aparARi havyam ..
    ayAcitAraM na hi devadevam adriH sutAM grAhayituM SaSAka .
    aByarTanABaNgaBayena sADur mADyasTyam izwe 'py avalambate 'rTe ..
    yadEva pUrve janane SarIraM sA dakzarozAt sudatI sasarja .
    tadApraBfty eva vimuktasaNgaH patiH paSUnAm aparigraho 'BUt ..
    sa kfttivAsAs tapase yatAtmA gaNgApravAhokzitadevadAru .
    prasTaM himAdrer mfganABiganDi kiM cit kvaRatkiMnaram aDyuvAsa ..
    gaRA nameruprasavAvataMsA BUrjatvacaH sparSavatIr daDAnAH .
    manaHSilAvicCuritA nizeduH SEleyanadDezu SilAtalezu ..
    tuzArasaMGAtaSilAH KurAgrEH samulliKan darpakalaH kakudmAn .
    dfzwaH kaTaM cid gavayEr vivignEr asoQasiMhaDvanir unnanAda ..
    tatrAgnim ADAya samitsamidDaM svam eva mUrtyantaram azwamUrtiH .
    svayaM viDAtA tapasaH PalAnAm kenApi kAmena tapaS cacAra ..
    anarGyam arGyeRa tam adrinATaH svargOkasAm arcitam arcayitvA .
    ArADanAyAsya saKIsametAM samAdideSa prayatAM tanUjAm ..
    pratyarTiBUtAm api tAM samADeH SuSrUzamARAM giriSo 'numene .
    vikArahetO sati vikriyante yezAM na cetAMsi ta eva DIrAH ..
    avacitabalipuzpA vedisaMmArgadakzA niyamaviDijalAnAM barhizAM copanetrI .
    giriSam upacacAra pratyahaM sA sukeSI niyamitapariKedA tacCiraScandrapAdEH .."
        .to_string();

    let mut converted = convert::convert_roman_to_indic(&input, slp1, devanagari);

    println!("{}\n\n{}\n\n", input, converted);

    // for i in converted.chars() {
    //     print!("{:?}", i);
    // }

    input = "अस्त्य् उत्तरस्यां दिशि देवतात्मा हिमालयो नाम नगाधिराजः ।
    पूर्वापरौ तोयनिधी विगाह्य स्थितः पृथिव्या इव मानदण्डः ॥
    यं सर्वशैलाः परिकल्प्य वत्सं मेरौ स्थिते दोग्धरि दोहदक्षे ।
    भास्वन्ति रत्नानि महौषधीश् च पृथूपदिष्टां दुदुहुर् धरित्रीम् ॥
    अनन्तरत्नप्रभवस्य यस्य हिमं न सौभाग्यविलोपि जातम् ।
    एको हि दोषो गुणसंनिपाते निमज्जतीन्दोः किरणेष्व् इवाङ्कः ॥
    यश् चाप्सरोविभ्रममण्डनानां संपादयित्रीं शिखरैर् बिभर्ति ।
    बलाहकच्छेदविभक्तरागाम् अकालसंध्याम् इव धातुमत्ताम् ॥
    आमेखलं संचरतां घनानां च्छायाम् अधःसानुगतां निषेव्य ।
    उद्वेजिता वृष्टिभिर् आश्रयन्ते शृङ्गाणि यस्यातपवन्ति सिद्धाः ॥
    पदं तुषारस्रुतिधौतरक्तं यस्मिन्न् अदृष्ट्वापि हतद्विपानाम् ।
    विदन्ति मार्गं नखरन्ध्रमुक्तैर् मुक्ताफलैः केसरिणां किराताः ॥
    न्यस्ताक्षरा धातुरसेन यत्र भूर्जत्वचः कुञ्जरबिन्दुशोणाः ।
    व्रजन्ति विद्याधरसुन्दरीणाम् अनङ्गलेखक्रिययोपयोगम् ॥
    यः पूरयन् कीचकरन्ध्रभागान् दरीमुखोत्थेन समीरणेन ।
    उद्गास्यताम् इच्छति किंनराणां तानप्रदायित्वम् इवोपगन्तुम् ॥
    कपोलकण्डूः करिभिर् विनेतुं विघट्टितानां सरलद्रुमाणाम् ।
    यत्र स्रुतक्षीरतया प्रसूतः सानूनि गन्धः सुरभीकरोति ॥
    वनेचराणां वनितासखानां दरीगृहोत्सङ्गनिषक्तभासः ।
    भवन्ति यत्रौषधयो रजन्याम् अतैलपूराः सुरतप्रदीपाः ॥
    उद्वेजयत्य् अङ्गुलिपार्ष्णिभागान् मार्गे शिलीभूतहिमे ऽपि यत्र ।
    न दुर्वहश्रोणिपयोधरार्ता भिन्दन्ति मन्दां गतिम् अश्वमुख्यः ॥
    दिवाकराद् रक्षति यो गुहासु लीनं दिवा भीतम् इवान्धकारम् ।
    क्षुद्रे ऽपि नूनं शरणं प्रपन्ने ममत्वम् उच्चैःशिरसां सतीव ॥
    लाङ्गूलविक्षेपविसर्पिशोभैर् इतस् ततश् चन्द्रमरीचिगौरैः ।
    यस्यार्थयुक्तं गिरिराजशब्दं कुर्वन्ति वालव्यजनैश् चमर्यः ॥
    यत्रांशुकाक्षेपविलज्जितानां यदृच्छया किंपुरुषाङ्गनानाम् ।
    दरीगृहद्वारविलम्बिबिम्बास् तिरस्करिण्यो जलदा भवन्ति ॥
    भागीरथीनिर्झरसीकराणां वोढा मुहुः कम्पितदेवदारुः ।
    यद् वायुर् अन्विष्टमृगैः किरातैर् आसेव्यते भिन्नशिखण्डिबर्हः ॥
    सप्तर्षिहस्तावचितावशेषाण्य् अधो विवस्वान् परिवर्तमानः ।
    पद्मानि यस्याग्रसरोरुहाणि प्रबोधयत्य् ऊर्ध्वमुखैर् मयूखैः ॥
    यज्ञाङ्गयोनित्वम् अवेक्ष्य यस्य सारं धरित्रीधरणक्षमं च ।
    प्रजापतिः कल्पितयज्ञभागं शैलाधिपत्यं स्वयम् अन्वतिष्ठत् ॥
    स मानसीं मेरुसखः पितॄणां कन्यां कुलस्य स्थितये स्थितिज्ञः ।
    मेनां मुनीनाम् अपि माननीयाम् आत्मानुरूपां विधिनोपयेमे ॥
    कालक्रमेणाथ तयोः प्रवृत्ते स्वरूपयोग्ये सुरतप्रसङ्गे ।
    मनोरमं यौवनम् उद्वहन्त्या गर्भो ऽभवद् भूधरराजपत्न्याः ॥
    असूत सा नागवधूपभोग्यं मैनाकम् अम्भोनिधिबद्धसख्यम् ।
    क्रुद्धे ऽपि पक्षच्छिदि वृत्रशत्राव् अवेदनाज्ञं कुलिशक्षतानाम् ॥
    अथावमानेन पितुः प्रयुक्ता दक्षस्य कन्या भवपूर्वपत्नी ।
    सती सती योगविसृष्टदेहा तां जन्मने शैलवधूं प्रपेदे ॥
    सा भूधराणाम् अधिपेन तस्यां समाधिमत्याम् उदपादि भव्या ।
    सम्यक्प्रयोगाद् अपरिक्षतायां नीताव् इवोत्साहगुणेन संपत् ॥
    प्रसन्नदिक् पांसुविविक्तवातं शङ्खस्वनानन्तरपुष्पवृष्टि ।
    शरीरिणां स्थावरजङ्गमानां सुखाय तज्जन्मदिनं बभूव ॥
    तया दुहित्रा सुतरां सवित्री स्फुरत्प्रभामण्डलया चकासे ।
    विदूरभूमिर् नवमेघशब्दाद् उद्भिन्नया रत्नशलाकयेव ॥
    दिने दिने सा परिवर्धमाना लब्धोदया चान्द्रमसीव लेखा ।
    पुपोष लावण्यमयान् विशेषाञ् ज्योत्स्नान्तराणीव कलान्तराणि ॥
    तां पार्वतीत्य् आभिजनेन नाम्ना बन्धुप्रियां बन्धुजनो जुहाव ।
    उ मेति मात्रा तपसो निषिद्धा पश्चाद् उमाख्यां सुमुखी जगाम ॥
    महीभृतः पुत्रवतो ऽपि दृष्टिस् तस्मिन्न् अपत्ये न जगाम तृप्तिम् ।
    अनन्तपुष्पस्य मधोर् हि चूते द्विरेफमाला सविशेषसङ्गा ॥
    प्रभामहत्या शिखयेव दीपस् त्रिमार्गयेव त्रिदिवस्य मार्गः ।
    संस्कारवत्येव गिरा मनीषी तया स पूतश् च विभूषितश् च ॥
    मन्दाकिनीसैकतवेदिकाभिः सा कन्दुकैः कृत्रिमपुत्रकैश् च ।
    रेमे मुहुर् मध्यगता सखीनां क्रीडारसं निर्विशतीव बाल्ये ॥
    तां हंसमालाः शरदीव गङ्गां महौषधिं नक्तम् इवात्मभासः ।
    स्थिरोपदेशाम् उपदेशकाले प्रपेदिरे प्राक्तनजन्मविद्याः ॥
    असंभृतं मण्डनम् अङ्गयष्टेर् अनासवाख्यं करणं मदस्य ।
    कामस्य पुष्पव्यतिरिक्तम् अस्त्रं बाल्यात् परं साथ वयः प्रपेदे ॥
    उन्मीलितं तूलिकयेव चित्रं सूर्यांशुभिर् भिन्नम् इवारविन्दम् ।
    बभूव तस्याश् चतुरस्रशोभि वपुर् विभक्तं नवयौवनेन ॥
    अभ्युन्नताङ्गुष्ठनखप्रभाभिर् निक्षेपणाद् रागम् इवोद्गिरन्तौ ।
    आजह्रतुस् तच्चरणौ पृथिव्यां स्थलारविन्दश्रियम् अव्यवस्थाम् ॥
    सा राजहंसैर् इव संनताङ्गी गतेषु लीलाञ्चितविक्रमेषु ।
    व्यनीयत प्रत्युपदेशलुब्धैर् आदित्सुभिर् नूपुरसिञ्जितानि ॥
    वृत्तानुपूर्वे च न चातिदीर्घे जङ्घे शुभे सृष्टवतस् तदीये ।
    शेषाङ्गनिर्माणविधौ विधातुर् लावण्य उत्पाद्य इवास यत्नः ॥
    नागेन्द्रहस्तास् त्वचि कर्कशत्वाद् एकान्तशैत्यात् कदलीविशेषाः ।
    लब्ध्वापि लोके परिणाहि रूपं जातास् तदूर्वोर् उपमानबाह्याः ॥
    एतावता नन्व् अनुमेयशोभं काञ्चीगुणस्थानम् अनिन्दितायाः ।
    आरोपितं यद् गिरिशेन पश्चाद् अनन्यनारीकमनीयम् अङ्कम् ॥
    तस्याः प्रविष्टा नतनाभिरन्ध्रं रराज तन्वी नवलोमराजिः ।
    नीवीम् अतिक्रम्य सितेतरस्य तन्मेखलामध्यमणेर् इवार्चिः ॥
    मध्येन सा वेदिविलग्नमध्या वलित्रयं चारु बभार बाला ।
    आरोहणार्थं नवयौवनेन कामस्य सोपानम् इव प्रयुक्तम् ॥
    अन्योन्यम् उत्पीडयद् उत्पलाक्ष्याः स्तनद्वयं पाण्डु तथा प्रवृद्धम् ।
    मध्ये यथा श्याममुखस्य तस्य मृणालसूत्रान्तरम् अप्य् अलभ्यम् ॥
    शिरीषपुष्पाधिकसौकुमार्यौ बाहू तदीयाव् इति मे वितर्कः ।
    पराजितेनापि कृतौ हरस्य यौ कण्ठपाशौ मकरध्वजेन ॥
    कण्ठस्य तस्याः स्तनबन्धुरस्य मुक्ताकलापस्य च निस्तलस्य ।
    अन्योन्यशोभाजननाद् बभूव साधारणो भूषणभूष्यभावः ॥
    चन्द्रं गता पद्मगुणान् न भुङ्क्ते पद्माश्रिता चान्द्रमसीम् अभिख्याम् ।
    उमामुखं तु प्रतिपद्य लोला द्विसंश्रयां प्रीतिम् अवाप लक्ष्मीः ॥
    पुष्पं प्रवालोपहितं यदि स्यान् मुक्ताफलं वा स्फुटविद्रुमस्थम् ।
    ततो ऽनुकुर्याद् विशदस्य तस्यास् ताम्रौष्ठपर्यस्तरुचः स्मितस्य ॥
    स्वरेण तस्याम् अमृतस्रुतेव प्रजल्पितायाम् अभिजातवाचि ।
    अप्य् अन्यपुष्टा प्रतिकूलशब्दा श्रोतुर् वितन्त्रीर् इव ताड्यमाना ॥
    प्रवातनीलोत्पलनिर्विशेषम् अधीरविप्रेक्षितम् आयताक्ष्या ।
    तया गृहीतं नु मृगाङ्गनाभ्यस् ततो गृहीतं नु मृगाङ्गनाभिः ॥
    तस्याः शलाकाञ्जननिर्मितेव कान्तिर् भ्रुवोर् आनतलेखयोर् या ।
    तां वीक्ष्य लीलाचतुराम् अनङ्गः स्वचापसौन्दर्यमदं मुमोच ॥
    लज्जा तिरश्चां यदि चेतसि स्याद् असंशयं पर्वतराजपुत्र्याः ।
    तं केशपाशं प्रसमीक्ष्य कुर्युर् वालप्रियत्वं शिथिलं चमर्यः ॥
    सर्वोपमाद्रव्यसमुच्चयेन यथाप्रदेशं विनिवेशितेन ।
    सा निर्मिता विश्वसृजा प्रयत्नाद् एकस्थसौन्दर्यदिदृक्षयेव ॥
    तां नारदः कामचरः कदा चित् कन्यां किल प्रेक्ष्य पितुः समीपे ।
    समादिदेशैकवधूं भवित्रीं प्रेम्णा शरीरार्धहरां हरस्य ॥
    गुरुः प्रगल्भे ऽपि वयस्य् अतो ऽस्यास् तस्थौ निवृत्तान्यवराभिलाषः ।
    ऋते कृशानोर् न हि मन्त्रपूतम् अर्हन्ति तेजांस्य् अपराणि हव्यम् ॥
    अयाचितारं न हि देवदेवम् अद्रिः सुतां ग्राहयितुं शशाक ।
    अभ्यर्थनाभङ्गभयेन साधुर् माध्यस्थ्यम् इष्टे ऽप्य् अवलम्बते ऽर्थे ॥
    यदैव पूर्वे जनने शरीरं सा दक्षरोषात् सुदती ससर्ज ।
    तदाप्रभृत्य् एव विमुक्तसङ्गः पतिः पशूनाम् अपरिग्रहो ऽभूत् ॥
    स कृत्तिवासास् तपसे यतात्मा गङ्गाप्रवाहोक्षितदेवदारु ।
    प्रस्थं हिमाद्रेर् मृगनाभिगन्धि किं चित् क्वणत्किंनरम् अध्युवास ॥
    गणा नमेरुप्रसवावतंसा भूर्जत्वचः स्पर्शवतीर् दधानाः ।
    मनःशिलाविच्छुरिता निषेदुः शैलेयनद्धेषु शिलातलेषु ॥
    तुषारसंघातशिलाः खुराग्रैः समुल्लिखन् दर्पकलः ककुद्मान् ।
    दृष्टः कथं चिद् गवयैर् विविग्नैर् असोढसिंहध्वनिर् उन्ननाद ॥
    तत्राग्निम् आधाय समित्समिद्धं स्वम् एव मूर्त्यन्तरम् अष्टमूर्तिः ।
    स्वयं विधाता तपसः फलानाम् केनापि कामेन तपश् चचार ॥
    अनर्घ्यम् अर्घ्येण तम् अद्रिनाथः स्वर्गौकसाम् अर्चितम् अर्चयित्वा ।
    आराधनायास्य सखीसमेतां समादिदेश प्रयतां तनूजाम् ॥
    प्रत्यर्थिभूताम् अपि तां समाधेः शुश्रूषमाणां गिरिशो ऽनुमेने ।
    विकारहेतौ सति विक्रियन्ते येषां न चेतांसि त एव धीराः ॥
    अवचितबलिपुष्पा वेदिसंमार्गदक्षा नियमविधिजलानां बर्हिषां चोपनेत्री ।
    गिरिशम् उपचचार प्रत्यहं सा सुकेशी नियमितपरिखेदा तच्छिरश्चन्द्रपादैः ॥"
    .to_string();

    converted = convert::convert_indic_to_roman(&input, devanagari, slp1);

    println!("\n\n{}\n\n{}\n\n", input, converted);

    // let input: String = "asty uttarasyAM diSi devatAtmA himAlayo nAma nagADirAjaH . pUrvAparO toyaniDI vigAhya sTitaH pfTivyA iva mAnadaRqaH .."
    //     .to_string();

    // let converted = convert::convert_roman_to_indic(&input, slp1, telugu);

    // println!("{}\n\n{}", input, converted);

    // let input: String = "అస్త్య్ ఉత్తరస్యాం దిశి దేవతాత్మా హిమాలయో నామ నగాధిరాజః ।
    // పూర్వాపరౌ తోయనిధీ విగాహ్య స్థితః పృథివ్యా ఇవ మానదణ్డః ॥"
    // .to_string();

    // let converted = convert::convert_indic_to_roman(&input, telugu, slp1);

    // println!("{}\n\n{}", input, converted);
}
